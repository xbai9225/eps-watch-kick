// use isis-eps-api::*;
use command_id::*;
use rust_udp::*;
use cubeos_service::Command;
use std::thread;
use std::time::Duration;

// use std::convert::TryFrom;

// CommandID Enum from Service can not be imported here, 
// copy-paste list of commands from service into this CommandID macro,
// to create a copy that can be used in the App
command_id!{
    //Keep these command IDs reserved, these commands are implemented in the CubeOS service
    Ping,
    Get,
    Set,
    //Add your commands here
    EpsPing,
    SystemStatus,
    OvercurrentState,
    AbfState,
    PiuHk,
    SysReset,
    ShutDownAll,
    WatchdogReset,
}

// This example App 
// - pings the Example Service
// - performs a Telemetry request
// - overwrites variable of the service
// - performs a second Telemetry request to check the overwritten data
fn main() -> Result<(),CubeOSError>{

    // App IP + Service IP
    let app_host = "127.0.0.1:9029".to_string();
    let service = "127.0.0.1:8021".to_string();

    // The App opens a UDP Connection by binding the app_host IP
    // This connection can then be used to transfer commands
    let connection = Connection::from_path(app_host,service);


 
    // Send command to Service and wait for reply
    //      
    // connection.transfer(command: Vec<u8>, rx_len: usize) -> Result<Vec<u8>>
    //
    // # Arguments:
    // command: Vec<u8> - Serialized Command to send to the service/payload
    // rx_len: usize - Length of read buffer/expected length of reply
    // 
    // # Output:
    // cubeos_error::Result<Vec<u8>>
    //
    // Output can be deserialized to any API struct or enum with
    // bincode::deserialize<E>(output)
    // where E is a struct or enum defined in the API
    //
    loop{
        // Create a command
        // 
        // Arguments:
        // `CommandID` -> command called
        // `Input` -> Input for Command, here ()

        let msg = Command::<CommandID,()>::serialize(CommandID::WatchdogReset,())?;
        println!("{:?}",msg);

        match connection.transfer(msg) {
            Ok(r) => println!("{:?}",r),
            Err(_) => println!("Error"),
        }

        thread::sleep(Duration::from_secs(60));
    }
    
    // Ok(())
}
