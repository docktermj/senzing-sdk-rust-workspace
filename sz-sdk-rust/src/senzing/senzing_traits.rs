use std::error::Error;

pub trait SzAbstractFactory {
    fn close(&self) -> Result<(), Box<dyn Error>>;
    fn create_config_manager(&self) -> Result<Box<dyn SzConfigManager>, Box<dyn Error>>;
    fn create_diagnostic(&self) -> Result<Box<dyn SzDiagnostic>, Box<dyn Error>>;
    fn create_engine(&self) -> Result<Box<dyn SzEngine>, Box<dyn Error>>;
    fn create_product(&self) -> Result<Box<dyn SzProduct>, Box<dyn Error>>;
    fn reinitialize(&self, config_id: i64) -> Result<(), Box<dyn Error>>;
}

pub trait SzConfig {
    fn export(&self) -> Result<String, Box<dyn Error>>;
    fn get_data_source_registry(&self) -> Result<String, Box<dyn Error>>;
    fn register_data_source(&self, data_source_code: &str) -> Result<String, Box<dyn Error>>;
    fn unregister_data_source(&self, data_source_code: &str) -> Result<String, Box<dyn Error>>;
}

pub trait SzConfigManager {
    fn create_config_from_config_id(
        &self,
        config_id: i64,
    ) -> Result<Box<dyn SzConfig>, Box<dyn Error>>;
    fn create_config_from_string(
        &self,
        config_definition: &str,
    ) -> Result<Box<dyn SzConfig>, Box<dyn Error>>;
    fn create_config_from_template(&self) -> Result<Box<dyn SzConfig>, Box<dyn Error>>;
    fn destroy(&self) -> Result<(), Box<dyn Error>>;
    fn get_config_registry(&self) -> Result<String, Box<dyn Error>>;
    fn get_default_config_id(&self) -> Result<i64, Box<dyn Error>>;
    fn register_config(
        &self,
        config_definition: &str,
        config_comment: &str,
    ) -> Result<i64, Box<dyn Error>>;
    fn replace_default_config_id(
        &self,
        current_default_config_id: i64,
        new_default_config_id: i64,
    ) -> Result<(), Box<dyn Error>>;
    fn set_default_config(
        &self,
        config_definition: &str,
        config_comment: &str,
    ) -> Result<i64, Box<dyn Error>>;
    fn set_default_config_id(&self, config_id: i64) -> Result<(), Box<dyn Error>>;
}

pub trait SzDiagnostic {
    fn check_repository_performance(&self, seconds_to_run: i32) -> Result<String, Box<dyn Error>>;
    fn destroy(&self) -> Result<(), Box<dyn Error>>;
    fn get_feature(&self, feature_id: i64) -> Result<String, Box<dyn Error>>;
    fn get_repository_info(&self) -> Result<String, Box<dyn Error>>;
}

pub trait SzEngine {
    fn add_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        record_definition: &str,
        flags: i64,
    ) -> Result<(), Box<dyn Error>>;
    fn close_export(&self, export_handle: usize) -> Result<(), Box<dyn Error>>;
    fn count_redo_records(&self) -> Result<i64, Box<dyn Error>>;
    fn delete_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<(), Box<dyn Error>>;
    fn destroy(&self) -> Result<(), Box<dyn Error>>;
    fn export_csv_entity_report(
        &self,
        csv_column_list: &str,
        flags: i64,
    ) -> Result<usize, Box<dyn Error>>;
    fn export_json_entity_report(&self, flags: i64) -> Result<usize, Box<dyn Error>>;
    fn fetch_next(&self, export_handle: usize) -> Result<String, Box<dyn Error>>;
    fn find_interesting_entities_by_entity_id(
        &self,
        entity_id: i64,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn find_interesting_entities_by_record_id(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn find_network_by_entity_id(
        &self,
        entity_id_list: &str,
        max_degrees: i32,
        build_out_degrees: i32,
        max_entities: i32,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn find_network_by_record_id(
        &self,
        record_list: &str,
        max_degrees: i32,
        build_out_degrees: i32,
        max_entities: i32,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn find_path_by_entity_id(
        &self,
        start_entity_id: i64,
        end_entity_id: i64,
        max_degrees: i32,
        avoid_entity_id_list: &str,
        required_data_sources_list: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn find_path_by_record_id(
        &self,
        start_data_source_code: &str,
        start_record_id: &str,
        end_data_source_code: &str,
        end_record_id: &str,
        max_degrees: i32,
        avoid_record_list: &str,
        required_data_sources_list: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn get_active_config_id(&self) -> Result<i64, Box<dyn Error>>;
    fn get_entity_by_entity_id(&self, entity_id: i64, flags: i64)
    -> Result<String, Box<dyn Error>>;
    fn get_entity_by_record_id(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn get_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn get_redo_record(&self) -> Result<String, Box<dyn Error>>;
    fn get_stats(&self) -> Result<String, Box<dyn Error>>;
    fn get_virtual_entity_by_record_id(
        &self,
        record_list: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn how_entity_by_entity_id(&self, entity_id: i64, flags: i64)
    -> Result<String, Box<dyn Error>>;
    fn preprocess_record(
        &self,
        record_definition: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn prime_engine(&self) -> Result<(), Box<dyn Error>>;
    fn process_redo_record(&self, redo_record: &str, flags: i64) -> Result<String, Box<dyn Error>>;
    fn reevaluate_entity(&self, entity_id: i64, flags: i64) -> Result<(), Box<dyn Error>>;
    fn reevaluate_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<(), Box<dyn Error>>;
    fn search_by_attributes(
        &self,
        attributes: &str,
        search_profile: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn stream_export_csv_entity_report(
        &self,
        csv_column_list: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn stream_export_json_entity_report(&self, flags: i64) -> Result<String, Box<dyn Error>>;
    fn why_entities(
        &self,
        entity_id_1: i64,
        entity_id_2: i64,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn why_record_in_entity(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
    fn why_records(
        &self,
        data_source_code_1: &str,
        record_id_1: &str,
        data_source_code_2: &str,
        record_id_2: &str,
        flags: i64,
    ) -> Result<String, Box<dyn Error>>;
}

pub trait SzProduct {
    fn get_license(&self) -> Result<String, Box<dyn Error>>;
    fn get_version(&self) -> Result<String, Box<dyn Error>>;
}
