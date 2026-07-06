pub mod download;
pub mod queue;
pub mod scheduler;
pub mod file;
pub mod settings;
pub mod statistics;
pub mod notification;

pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        download::start_download,
        download::pause_download,
        download::resume_download,
        download::cancel_download,
        download::restart_download,
        download::delete_download,
        download::verify_download,
        
        queue::add_to_queue,
        queue::remove_from_queue,
        queue::reorder_queue,
        queue::clear_queue,
        queue::start_queue,
        queue::pause_queue,
        
        scheduler::create_schedule,
        scheduler::update_schedule,
        scheduler::delete_schedule,
        scheduler::enable_schedule,
        scheduler::disable_schedule,
        
        file::open_file,
        file::open_folder,
        file::rename_file,
        file::move_file,
        file::delete_file,
        
        settings::get_settings,
        settings::save_settings,
        settings::reset_settings,
        settings::import_settings,
        settings::export_settings,
        
        statistics::get_statistics,
        statistics::reset_statistics,
        
        notification::get_notifications,
        notification::mark_as_read,
        notification::clear_notifications
    ]
}
