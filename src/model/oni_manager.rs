pub use nite2::NiteUserId;
use nite2::UserTracker;
use openni2::{Device, OniRGB888Pixel, SensorType, Stream};

pub struct OniManager {
	#[allow(dead_code)]
	device: &'static Device,
	depth_stream: &'static Stream<'static>,
	user_tracker: &'static mut UserTracker<'static>,
	user_map: &'static mut [NiteUserId],
	users_present: bool,
	user_map_width: usize,
	user_map_height: usize,
	user_map_offset_x: f32,
	user_map_offset_y: f32,
	user_map_scale_x: f32,
	user_map_scale_y: f32,
	color_stream: &'static Stream<'static>,
	color_frame: &'static mut [OniRGB888Pixel],
	color_width: usize,
	color_height: usize,
}

impl OniManager {
	pub fn update(&mut self) -> Result<(), OniError> {
		let user_frame = self.user_tracker.read_frame()?;
		let user_map = user_frame.user_map();
		if self.user_map_width != user_map.width || self.user_map_height != user_map.height {
			println!(
				"Wrong user map dimensions! Expected {}x{}, got {}x{}",
				self.user_map_width, self.user_map_height, user_map.width, user_map.height
			);
			// FIXME obviously this is not "ok"
			return Ok(());
		}

		self.users_present = !user_frame.users().is_empty();
		self.user_map.copy_from_slice(user_map.pixels);

		let color_frame = self.color_stream.read_frame::<OniRGB888Pixel>()?;
		self.color_frame.copy_from_slice(color_frame.pixels());

		Ok(())
	}

	pub fn is_anyone_here(&self) -> bool {
		self.users_present
	}

	pub fn user_at_coords(&self, board_pct_x: f32, board_pct_y: f32) -> NiteUserId {
		let pct_x = (board_pct_x - self.user_map_offset_x) * self.user_map_scale_x;
		let pct_y = (board_pct_y - self.user_map_offset_y) * self.user_map_scale_y;
		if pct_y < 0.0 || pct_y > 1.0 || pct_x < 0.0 || pct_x > 1.0 {
			return 0;
		}
		let row = (pct_y * (self.user_map_height - 1) as f32) as usize;
		let col = (pct_x * (self.user_map_width - 1) as f32) as usize;
		let idx = row * self.user_map_width + col;
		self.user_map[idx]
	}

	pub fn color_frame(&self) -> (&[OniRGB888Pixel], usize, usize) {
		(self.color_frame, self.color_width, self.color_height)
	}

	pub fn user_map(&self) -> (&[NiteUserId], usize, usize) {
		(self.user_map, self.user_map_width, self.user_map_height)
	}

	pub fn create(board_width: usize, board_height: usize) -> Result<OniManager, OniError> {
		openni2::init()?;
		nite2::init()?;

		let default_device = Box::leak(Box::new(Device::open_default()?));
		let depth_stream = Box::leak(Box::new(default_device.create_stream(SensorType::DEPTH)?));
		let depth_mode = depth_stream.get_video_mode()?;
		println!("{:?}", depth_mode);
		let user_map_width = depth_mode.resolution_x as usize;
		let user_map_height = depth_mode.resolution_y as usize;
		let user_tracker = UserTracker::open_default()?;

		let color_stream = default_device.create_stream(SensorType::COLOR)?;
		color_stream.start()?;
		let color_mode = color_stream.get_video_mode()?;
		let color_width = color_mode.resolution_x as usize;
		let color_height = color_mode.resolution_y as usize;
		println!("{:?}", color_mode);
		if let Err(e) = default_device.set_image_registration(true) {
			println!("Failed to set depth/color registration: {:?}", e);
		}

		let (user_map_offset_x, user_map_offset_y, user_map_scale_x, user_map_scale_y) = {
			let board_ratio = board_width as f32 / board_height as f32;
			let stream_ratio = user_map_width as f32 / user_map_height as f32;

			if stream_ratio > board_ratio {
				// When stream is wider than board, align it to the board's bottom
				(
					0.0,
					1.0 - (board_ratio * stream_ratio.recip()),
					1.0,
					stream_ratio / board_ratio,
				)
			} else {
				// When stream is narrower than board, center it horizontally
				// NO fucking clue if this works
				(
					(1.0 - (stream_ratio * board_ratio.recip())) * 0.5,
					0.0,
					board_ratio / stream_ratio,
					1.0,
				)
			}
		};

		// println!(
		// 	"Given board of {}x{} and user map of {}x{},\nposition user map at {},{} and scale coords {},{}",
		// 	board_width, board_height, user_map_width, user_map_height,
		// 	user_map_offset_x, user_map_offset_y, user_map_scale_x, user_map_scale_y
		// );

		Ok(OniManager {
			device: default_device,
			depth_stream,
			user_tracker: Box::leak(Box::new(user_tracker)),
			user_map: vec![0; user_map_width * user_map_height].leak(),
			users_present: false,
			user_map_width,
			user_map_height,
			user_map_offset_x,
			user_map_offset_y,
			user_map_scale_x,
			user_map_scale_y,
			color_stream: Box::leak(Box::new(color_stream)),
			color_frame: vec![
				OniRGB888Pixel { r: 0, g: 0, b: 0 };
				color_mode.resolution_x as usize * color_mode.resolution_y as usize
			]
			.leak(),
			color_width,
			color_height,
		})
	}
}

impl Drop for OniManager {
	fn drop(&mut self) {
		self.user_tracker.shutdown();
		self.depth_stream.stop();
		self.color_stream.stop();
		self.device.close();
		let _ = nite2::shutdown();
		let _ = openni2::shutdown();
	}
}

#[derive(Clone)]
pub enum OniError {
	OpenNi(openni2::Status),
	Nite(nite2::Status),
}

impl std::fmt::Debug for OniError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			OniError::OpenNi(openni2::Status::Error(string)) => {
				fmt.debug_tuple("OniError::OpenNi").field(&string).finish()
			}
			OniError::OpenNi(status) => {
				fmt.write_fmt(format_args!("OniError::OpenNi({:?})", status))
			}
			OniError::Nite(status) => fmt.write_fmt(format_args!("OniError::Nite({:?})", status)),
		}
	}
}

impl From<openni2::Status> for OniError {
	fn from(status: openni2::Status) -> Self {
		OniError::OpenNi(status)
	}
}

impl From<nite2::Status> for OniError {
	fn from(status: nite2::Status) -> Self {
		OniError::Nite(status)
	}
}
