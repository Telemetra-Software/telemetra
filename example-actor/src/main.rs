use std::error::Error;

// TODO: Move to SDK
mod telemetra {
    
    use std::error::Error;
    use std::marker::PhantomData;
    use std::string::String;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::fmt::{Write, Display, Formatter, Result};
    use std::fmt;
    use std::vec::Vec;

    pub type TelemetraKey = std::string::String;

    pub enum TelemetraValue {
        SInteger(i32),
        UInteger(u32),
        String(std::string::String),
    }

    impl fmt::Display for TelemetraValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TelemetraValue::SInteger(val) => write!(f, "{val}"),
                TelemetraValue::UInteger(val) => write!(f, "{val}"),
                TelemetraValue::String(val) => write!(f, "{val}"),
            }
        }
    }

    
    pub trait TelemetraSerialize {
        fn telemetra_serialize(&self) ->
            std::result::Result<std::vec::Vec<(TelemetraKey,
                                              TelemetraValue)>,
                   Box<dyn Error>>;
    }
    
    pub struct TelemetraService<T: TelemetraSerialize> {
        name: &'static str,
        port: i32,
        _phantom: PhantomData<T>,
    }

    impl<T: TelemetraSerialize> TelemetraService<T> {
        pub fn send(&self, data: &T) -> std::result::Result<(), Box<dyn Error>> {
            
            logger::log_info!("[{name}] Sending data", name = self.name);

            //
            // Default fields
            //
            
            let mut msg = String::new();
            write!(&mut msg, "service: {name}\n", name = self.name);

            let now = SystemTime::now();
            let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
            let unix_time = duration_since_epoch.as_secs();
            write!(&mut msg, "time: {unix_time}\n");

            //
            // Custom fields
            //

            let vec = data.telemetra_serialize()?;
            for (key, val) in vec {
                write!(&mut msg, "{key}: {val}\n");
            }
            
            
            logger::log_info!("[{name}] Message:\n{msg}", name = self.name);
            
            // TODO
            
            Ok(())
        }

        pub fn await_connection(self) -> std::result::Result<Self, Box<dyn Error>> {

            logger::log_info!("[{name}] Waiting for connection on port {port}",
                              name = self.name, port = self.port);
            
            // TODO
            
            Ok(self)
        }

        pub fn await_execution(self) -> std::result::Result<Self, Box<dyn Error>> {

            logger::log_info!("[{name}] Waiting for execution", name = self.name);
            
            // TODO

            Ok(self)
        }

        pub fn stop(self) -> std::result::Result<(), Box<dyn Error>> {
            
            logger::log_info!("[{name}] Stopping", name = self.name);

            // TODO

            Ok(())
        }
    }

    pub fn new_service<T: TelemetraSerialize>(name: &'static str, port: i32)->
        std::result::Result<TelemetraService<T>, Box<dyn Error>> {
            Ok(TelemetraService::<T>{
            name: name,
            port: port,
            _phantom: PhantomData,
        })
    }
    
    pub mod logger {
        
        macro_rules! log_info {
            ($($arg:tt)*) => {
                println!("[INFO] {}", format!($($arg)*));
            }
        }

        macro_rules! log_warn {
            ($($arg:tt)*) => {
                println!("[WARN] {}", format!($($arg)*));
            }
        }
        
        macro_rules! log_error {
            ($($arg:tt)*) => {
                println!("[ERROR] {}", format!($($arg)*));
            }
        }

        pub(crate) use log_info;
        pub(crate) use log_warn;
        pub(crate) use log_error;
        
    }
}

use crate::telemetra::{TelemetraSerialize, TelemetraKey, TelemetraValue};
use std::vec::Vec;
use std::result::Result;

struct ActorData {
    n: i32,
}

impl TelemetraSerialize for ActorData {
    
    fn telemetra_serialize(&self) ->
        Result<Vec<(TelemetraKey, TelemetraValue)>, Box<dyn Error>>
    {        
        let mut vec = Vec::<(TelemetraKey, TelemetraValue)>::new();
        vec.push((String::from("n"), TelemetraValue::SInteger(self.n)));

        return Ok(vec);
    }
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!("Hello form an actor!");

    let service =
        telemetra::new_service::<ActorData>("service/iteration-number", 8090)?
        .await_connection()?
        .await_execution()?;
    
    for i in 0..10 {
        service.send(&ActorData{n: i})?;
    }

    service.stop()?;

    Ok(())
}
