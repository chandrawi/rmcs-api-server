pub mod model;
pub mod device;
pub mod group;
pub mod set;
pub mod data;
pub mod buffer;
pub mod slice;
pub mod log;

// model service procedure names
const READ_MODEL: &str = "read_model";
const CREATE_MODEL: &str = "create_model";
const UPDATE_MODEL: &str = "update_model";
const DELETE_MODEL: &str = "delete_model";
const READ_MODEL_CONFIG: &str = "read_model_config";
const CREATE_MODEL_CONFIG: &str = "create_model_config";
const UPDATE_MODEL_CONFIG: &str = "update_model_config";
const DELETE_MODEL_CONFIG: &str = "delete_model_config";
// device service procedure names
const READ_DEVICE: &str = "read_device";
const CREATE_DEVICE: &str = "create_device";
const UPDATE_DEVICE: &str = "update_device";
const DELETE_DEVICE: &str = "delete_device";
const READ_DEVICE_CONFIG: &str = "read_device_config";
const CREATE_DEVICE_CONFIG: &str = "create_device_config";
const UPDATE_DEVICE_CONFIG: &str = "update_device_config";
const DELETE_DEVICE_CONFIG: &str = "delete_device_config";
const READ_TYPE: &str = "read_type";
const CREATE_TYPE: &str = "create_type";
const UPDATE_TYPE: &str = "update_type";
const DELETE_TYPE: &str = "delete_type";
const CHANGE_TYPE_MODEL: &str = "change_type_model";
// group service procedure names
const READ_GROUP: &str = "read_group";
const CREATE_GROUP: &str = "create_group";
const UPDATE_GROUP: &str = "update_group";
const DELETE_GROUP: &str = "delete_group";
const CHANGE_GROUP_MEMBER: &str = "change_group_member";
// set service procedure names
const READ_SET: &str = "read_set";
const CREATE_SET: &str = "create_set";
const UPDATE_SET: &str = "update_set";
const DELETE_SET: &str = "delete_set";
const CHANGE_SET_MEMBER: &str = "change_set_member";
// data service procedure names
const READ_DATA: &str = "read_data";
const CREATE_DATA: &str = "create_data";
const DELETE_DATA: &str = "delete_data";
// buffer service procedure names
const READ_BUFFER: &str = "read_buffer";
const CREATE_BUFFER: &str = "create_buffer";
const UPDATE_BUFFER: &str = "update_buffer";
const DELETE_BUFFER: &str = "delete_buffer";
// slice service procedure names
const READ_SLICE: &str = "read_slice";
const CREATE_SLICE: &str = "create_slice";
const UPDATE_SLICE: &str = "update_slice";
const DELETE_SLICE: &str = "delete_slice";
// log service procedure names
const READ_LOG: &str = "read_log";
const CREATE_LOG: &str = "create_log";
const UPDATE_LOG: &str = "update_log";
const DELETE_LOG: &str = "delete_log";
