pub type uint64_t = ::c_ulonglong;
//bl_mcu_sdk lhal interface
extern "C"{
    pub fn bflb_mtimer_get_time_us()->uint64_t; 
}