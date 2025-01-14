pub use nite2::NiteUserId;
use nite2::UserTracker;
use openni2::{Device, OniRGB888Pixel, PixelFormat, SensorType, Stream, VideoMode};

pub struct OniManager {
	#[allow(dead_code)]
	device: &'static Device,
	depth_stream: &'static Stream<'static>,
	user_tracker: &'static mut UserTracker<'static>,
	user_map: &'static mut [NiteUserId],
	users_present: bool,
	stream_width: usize,
	stream_height: usize,
	stream_offset_x: f32,
	stream_offset_y: f32,
	stream_scale_x: f32,
	stream_scale_y: f32,
	color_stream: &'static Stream<'static>,
	color_frame: &'static mut [u8],
}

impl OniManager {
	pub fn update(&mut self) -> Result<(), OniError> {
		let user_frame = self.user_tracker.read_frame()?;
		let user_map = user_frame.user_map();
		if self.stream_width != user_map.width || self.stream_height != user_map.height {
			println!(
				"Wrong user map dimensions! Expected {}x{}, got {}x{}",
				self.stream_width, self.stream_height, user_map.width, user_map.height
			);
			// FIXME obviously this is not "ok"
			return Ok(());
		}
		self.users_present = !user_frame.users().is_empty();
		self.user_map.copy_from_slice(user_map.pixels);

		let color_frame = self.color_stream.read_frame::<OniRGB888Pixel>()?;
		if self.color_frame.len() != color_frame.pixels().len() {
			println!(
				"Wrong color frame length! Expected {}, got {}",
				self.color_frame.len(),
				color_frame.pixels().len()
			);
			return Ok(());
		}
		for (i, pixel) in color_frame.pixels().iter().enumerate() {
			let mut value = pixel.r as u16 + pixel.g as u16 + pixel.b as u16;
			value /= 3;
			self.color_frame[i] = value as u8;
		}

		Ok(())
	}

	pub fn is_anyone_here(&self) -> bool {
		self.users_present
	}

	pub fn user_at_coords(&self, board_pct_x: f32, board_pct_y: f32) -> NiteUserId {
		match self.coords_to_idx(board_pct_x, board_pct_y) {
			Some(idx) => self.user_map[idx],
			None => 0,
		}
	}

	pub fn state_at_coords(&self, board_pct_x: f32, board_pct_y: f32) -> u8 {
		match self.coords_to_idx(board_pct_x, board_pct_y) {
			Some(idx) => self.color_frame[idx],
			None => 0,
		}
	}

	fn coords_to_idx(&self, board_pct_x: f32, board_pct_y: f32) -> Option<usize> {
		let pct_x = (board_pct_x - self.stream_offset_x) * self.stream_scale_x;
		let pct_y = (board_pct_y - self.stream_offset_y) * self.stream_scale_y;
		if pct_y < 0.0 || pct_y > 1.0 || pct_x < 0.0 || pct_x > 1.0 {
			return None;
		}
		let row = (pct_y * (self.stream_height - 1) as f32) as usize;
		let col = (pct_x * (self.stream_width - 1) as f32) as usize;
		let idx = row * self.stream_width + col;
		Some(idx)
	}

	pub fn create(board_width: usize, board_height: usize) -> Result<OniManager, OniError> {
		openni2::init()?;
		nite2::init()?;

		let default_device = Box::leak(Box::new(Device::open_default()?));
		let depth_stream = Box::leak(Box::new(default_device.create_stream(SensorType::DEPTH)?));
		let depth_mode = depth_stream.get_video_mode()?;
		let stream_width = depth_mode.resolution_x as usize;
		let stream_height = depth_mode.resolution_y as usize;
		let user_tracker = UserTracker::open_default()?;

		let color_stream = default_device.create_stream(SensorType::COLOR)?;
		let color_mode = VideoMode {
			pixel_format: PixelFormat::RGB888,
			resolution_x: 320,
			resolution_y: 240,
			fps: 30,
		};
		color_stream.set_video_mode(color_mode)?;
		color_stream.start()?;
		if let Err(e) = default_device.set_image_registration(true) {
			println!("Failed to set depth/color registration: {:?}", e);
		}

		let (stream_offset_x, stream_offset_y, stream_scale_x, stream_scale_y) = {
			let board_ratio = board_width as f32 / board_height as f32;
			let stream_ratio = stream_width as f32 / stream_height as f32;

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
		// 	board_width, board_height, stream_width, stream_height,
		// 	stream_offset_x, stream_offset_y, stream_scale_x, stream_scale_y
		// );

		Ok(OniManager {
			device: default_device,
			depth_stream,
			user_tracker: Box::leak(Box::new(user_tracker)),
			user_map: vec![0; stream_width * stream_height].leak(),
			users_present: false,
			stream_width,
			stream_height,
			stream_offset_x,
			stream_offset_y,
			stream_scale_x,
			stream_scale_y,
			color_stream: Box::leak(Box::new(color_stream)),
			color_frame: vec![
				0;
				color_mode.resolution_x as usize * color_mode.resolution_y as usize
			]
			.leak(),
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
