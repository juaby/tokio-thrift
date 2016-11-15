// DO NOT EDIT: autogenerated by tokio_thrift
#![allow(dead_code, unused_imports, non_snake_case, non_camel_case_types)]
use futures::{Future, Async};
use tokio_thrift::protocol::{ThriftDeserializer, ThriftSerializer, ThriftMessageType};
use tokio_thrift::protocol::{Error, ThriftType};
use tokio_thrift::protocol::{Serializer, Deserializer};
use tokio_thrift::protocol::{Deserialize, Serialize};
use tokio_thrift::framed_transport::*;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use tokio_proto::server;
use tokio_proto::easy::{self, pipeline};
use tokio_service::Service;

use std::io;
use std::net::SocketAddr;
use std::str::FromStr;


pub trait HelloService: Send {
    fn hello_name(&self, name: String) -> Box<Future<Item = String, Error = io::Error>>;
    fn hello(&self) -> Box<Future<Item = String, Error = io::Error>>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelloServiceMethods {
    Mhello_name,
    Mhello,
}

impl FromStr for HelloServiceMethods {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::HelloServiceMethods::*;
        match s {
            "hello_name" => Ok(Mhello_name),
            "hello" => Ok(Mhello),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift method data"))
        }
    }
}


impl ParseThrift for HelloServiceMethods {
    type Args = HelloServiceMethodArgs;
    type Ret = HelloServiceMethodReturn;
    fn parse_args<D: Deserializer + ThriftDeserializer>(&self, proto: &mut D) -> Result<Self::Args, Error> {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethods::*;
        match self {
            &Mhello_name => Hellohello_nameArgs::deserialize(proto).map(Ahello_name),
            &Mhello => HellohelloArgs::deserialize(proto).map(Ahello),
        }
    }

    fn parse_ret<D: Deserializer + ThriftDeserializer>(&self, proto: &mut D) -> Result<Self::Ret, Error> {
        use self::HelloServiceMethodReturn::*;
        use self::HelloServiceMethods::*;
        match self {
            &Mhello_name => String::deserialize(proto).map(Rhello_name),
            &Mhello => String::deserialize(proto).map(Rhello),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelloServiceMethodArgs {
    Ahello_name(Hellohello_nameArgs),
    Ahello(HellohelloArgs),
}


impl Serialize for HelloServiceMethodArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::HelloServiceMethodArgs::*;
        match self {
            &Ahello_name(ref b) => {
                try!(s.write_message_begin("hello_name", ThriftMessageType::Call));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
            &Ahello(ref b) => {
                try!(s.write_message_begin("hello", ThriftMessageType::Call));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelloServiceMethodReturn {
    Rhello_name(String),
    Rhello(String),

}


impl Serialize for HelloServiceMethodReturn {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::HelloServiceMethodReturn::*;
        match self {
            &Rhello_name(ref b) => {
                try!(s.write_message_begin("hello_name", ThriftMessageType::Reply));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
            &Rhello(ref b) => {
                try!(s.write_message_begin("hello", ThriftMessageType::Reply));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
        };
        Ok(())
    }
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hellohello_nameArgs {
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HellohelloArgs {
}
impl Serialize for Hellohello_nameArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        try!(s.write_struct_begin("Hello_hello_name_Args"));
        try!(s.write_field_begin("name", ThriftType::String, 1));
        try!(self.name.serialize(s));
        try!(s.write_field_end());
        try!(s.write_field_stop());
        try!(s.write_struct_end());
        Ok(())
    }
}
impl Serialize for HellohelloArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        try!(s.write_struct_begin("Hello_hello_Args"));
        
        try!(s.write_field_stop());
        try!(s.write_struct_end());
        Ok(())
    }
}




impl Deserialize for Hellohello_nameArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        try!(de.read_struct_begin());
        let mut name = None;
        loop {
            let scheme_field = try!(de.read_field_begin());
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {
                1 => {
                    if scheme_field.ty == ThriftType::String {
                        name = Some(try!(de.deserialize_str()));
                    } else {
                        // skip
                    }
                },
                _ => (),// skip
            }
            try!(de.read_field_end());
        };
        try!(de.read_struct_end());
        let args = Hellohello_nameArgs {
            name: name.unwrap(),
        };
        Ok(args)
    }
}

impl Deserialize for HellohelloArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        try!(de.read_struct_begin());
        
        loop {
            let scheme_field = try!(de.read_field_begin());
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {
                
                _ => (),// skip
            }
            try!(de.read_field_end());
        };
        try!(de.read_struct_end());
        let args = HellohelloArgs {
            
        };
        Ok(args)
    }
}


pub struct HelloClient {
    inner: easy::EasyClient<HelloServiceMethodArgs, HelloServiceMethodReturn>,
}

impl HelloClient {
    pub fn new(handle: &Handle, stream: TcpStream) -> Self {
        let transport = new_thrift_client_transport::<_, HelloServiceMethods, HelloServiceMethodArgs>(stream);
        let easy_client = easy::pipeline::connect(transport, handle);

        HelloClient { inner: easy_client }
    }
}

impl Service for HelloClient {
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, arg: Self::Request) -> Self::Future {
        self.inner.call(arg)
            .boxed()
    }
}

impl HelloService for HelloClient {
    fn hello_name(&self, name: String) -> Box<Future<Item = String, Error = io::Error>> {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethodReturn::*;
        let args = Hellohello_nameArgs {
            name: name,
        };
        let ret = self.call(Ahello_name(args)).map(|r| {
            if let Rhello_name(ret) = r {
                ret
            } else {
                unreachable!("generated code error. map be a bug.");
            }
        });
        Box::new(ret)
    }
    fn hello(&self) -> Box<Future<Item = String, Error = io::Error>> {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethodReturn::*;
        let args = HellohelloArgs {
        };
        let ret = self.call(Ahello(args)).map(|r| {
            if let Rhello(ret) = r {
                ret
            } else {
                unreachable!("generated code error. map be a bug.");
            }
        });
        Box::new(ret)
    }
}
#[derive(Clone)]
pub struct HelloServer<T>
{
    inner: T,
}

impl <T: HelloService>HelloServer<T>
{
    pub fn new(inner: T) -> Self
    {
        HelloServer {
            inner: inner
        }
    }

    pub fn serve(self, handle: &Handle,  addr: SocketAddr)
                    -> io::Result<server::ServerHandle>
        where T: HelloService+Clone+'static
    {
        server::listen(handle, addr, move |stream| {
            Ok(pipeline::EasyServer::new(self.clone(),
                                         new_thrift_server_transport::<_, HelloServiceMethods, HelloServiceMethodReturn>(stream)))
        })
    }

}

impl <T>Service for HelloServer<T>
    where T: HelloService
{
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Error = io::Error;
    type Future = Box<Future<Item = HelloServiceMethodReturn, Error = io::Error>>;


    fn call(&self, req: Self::Request) -> Self::Future {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethodReturn::*;
        match req {
            Ahello_name(_args)  => Box::new(self.inner.hello_name(
                _args.name,
            ).map(Rhello_name)),
            Ahello(_args)  => Box::new(self.inner.hello(
            ).map(Rhello)),
        }
    }
}
