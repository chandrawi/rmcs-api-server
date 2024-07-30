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

// operation error message
const MODEL_NOT_FOUND: &str = "requested model not found";
const MODEL_CREATE_ERR: &str = "create model error";
const MODEL_UPDATE_ERR: &str = "update model error";
const MODEL_DELETE_ERR: &str = "delete model error";
const ADD_TYPE_ERR: &str = "add model type error";
const RMV_TYPE_ERR: &str = "remove model type error";
const CFG_NOT_FOUND: &str = "requested config not found";
const CFG_CREATE_ERR: &str = "create config error";
const CFG_UPDATE_ERR: &str = "update config error";
const CFG_DELETE_ERR: &str = "delete config error";
const DEVICE_NOT_FOUND: &str = "requested device not found";
const DEVICE_CREATE_ERR: &str = "create device error";
const DEVICE_UPDATE_ERR: &str = "update device error";
const DEVICE_DELETE_ERR: &str = "delete device error";
const GATEWAY_NOT_FOUND: &str = "requested gateway not found";
const GATEWAY_CREATE_ERR: &str = "create gateway error";
const GATEWAY_UPDATE_ERR: &str = "update gateway error";
const GATEWAY_DELETE_ERR: &str = "delete gateway error";
const TYPE_NOT_FOUND: &str = "requested type not found";
const TYPE_CREATE_ERR: &str = "create type error";
const TYPE_UPDATE_ERR: &str = "update type error";
const TYPE_DELETE_ERR: &str = "delete type error";
const GROUP_NOT_FOUND: &str = "requested group not found";
const GROUP_CREATE_ERR: &str = "create group error";
const GROUP_UPDATE_ERR: &str = "update group error";
const GROUP_DELETE_ERR: &str = "delete group error";
const GROUP_ADD_ERR: &str = "add group member error";
const GROUP_RMV_ERR: &str = "remove group member error";
const SET_NOT_FOUND: &str = "requested set not found";
const SET_CREATE_ERR: &str = "create set error";
const SET_UPDATE_ERR: &str = "update set error";
const SET_DELETE_ERR: &str = "delete set error";
const SET_ADD_ERR: &str = "add set member error";
const SET_RMV_ERR: &str = "remove set member error";
const SET_SWP_ERR: &str = "swap set member position error";
const DATA_NOT_FOUND: &str = "requested data not found";
const DATA_CREATE_ERR: &str = "create data error";
const DATA_DELETE_ERR: &str = "delete data error";
const BUFFER_NOT_FOUND: &str = "requested buffer not found";
const BUFFER_CREATE_ERR: &str = "create buffer error";
const BUFFER_UPDATE_ERR: &str = "update buffer error";
const BUFFER_DELETE_ERR: &str = "delete buffer error";
const SLICE_NOT_FOUND: &str = "requested slice not found";
const SLICE_CREATE_ERR: &str = "create slice error";
const SLICE_UPDATE_ERR: &str = "update slice error";
const SLICE_DELETE_ERR: &str = "delete slice error";
const LOG_NOT_FOUND: &str = "requested log not found";
const LOG_CREATE_ERR: &str = "create log error";
const LOG_UPDATE_ERR: &str = "update log error";
const LOG_DELETE_ERR: &str = "delete log error";
