pub use nite2::NiteUserId;
use nite2::UserTrackerManager;
// pub use nite2::{UserMap, UserTrackerFrame};
use openni2::{Device, SensorType};

pub struct OniManager {
	user_tracker: &'static mut UserTrackerManager<'static>,
	pub user_map: &'static mut [NiteUserId],
	pub users: Vec<NiteUserId>,
	user_map_width: usize,
	user_map_height: usize,
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

		self.users = user_frame
			.users()
			.into_iter()
			.map(|user_data| user_data.id())
			.collect();
		self.user_map.copy_from_slice(user_map.pixels);
		Ok(())
	}

	pub fn index_at(&self, pct_x: f32, pct_y: f32) -> usize {
		let row = (pct_y * (self.user_map_height - 1) as f32) as usize;
		let col = (pct_x * (self.user_map_width - 1) as f32) as usize;
		row * self.user_map_width + col
	}

	pub fn create() -> Result<OniManager, OniError> {
		openni2::init()?;
		nite2::init()?;

		let default_device = Device::open_default()?;
		let depth_stream = default_device.create_stream(SensorType::DEPTH)?;
		let video_mode = depth_stream.get_video_mode()?;
		let user_map_width = video_mode.resolution_x as usize;
		let user_map_height = video_mode.resolution_y as usize;
		let mut user_tracker = UserTrackerManager::create()?;
		let _ = user_tracker.track_skeletons(false);

		Ok(OniManager {
			user_tracker: Box::leak(Box::new(user_tracker)),
			user_map: vec![0; user_map_width * user_map_height].leak(),
			users: vec![],
			user_map_width,
			user_map_height,
		})
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
