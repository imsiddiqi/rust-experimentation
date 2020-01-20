pub mod symmetric {

    use futures::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::net::Ipv4Addr;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::net::tcp::{ReadHalf, WriteHalf};
    use tokio_serde::SymmetricallyFramed;
    use tokio_serde::formats::*;
    use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

    #[derive(Debug)]
    pub enum Error<'a, T> where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        IO(std::io::Error),
        ReadError(<Reader<'a, T> as futures::TryStream>::Error),
        WriteError(<Writer<'a, T> as futures::Sink<T>>::Error),
    }

    pub type Reader<'a, T> = SymmetricallyFramed<
        FramedRead<ReadHalf<'a>, LengthDelimitedCodec>,
        T,
        SymmetricalBincode<T>>;

    pub type Writer<'a, T> = SymmetricallyFramed<
        FramedWrite<WriteHalf<'a>, LengthDelimitedCodec>,
        T,
        SymmetricalBincode<T>>;

    pub struct Receiver<'a, T> {
        reader: Reader<'a, T>,
    }

    impl<'a, T> Receiver<'a, T> where
        T: for<'de> Deserialize<'de> + Serialize,
        Reader<'a, T> : TryStream<Ok=T> + Unpin,
    {
        pub async fn recv(&mut self) -> Result<Option<T>, Error<'a, T>> {
            Ok(self.reader.try_next().await.map_err(Error::ReadError)?)
        }
    }

    pub struct Sender<'a, T> {
        writer: Writer<'a, T>,
    }

    impl<'a, T> Sender<'a, T> where
        T: for<'de> Deserialize<'de> + Serialize,
        Writer<'a, T> : Sink<T> + Unpin
    {
        pub async fn send(&mut self, item: T) -> Result<(), Error<'a, T>> {
            Ok(self.writer.send(item).await.map_err(Error::WriteError)?)
        }
    }

    pub struct Channel<T> {
        socket: TcpStream,
        ghost: std::marker::PhantomData<T>,
    }

    impl<T> Channel<T> where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        pub async fn connect(address: &Ipv4Addr, port: u16)
            -> Result<Channel<T>, Error<'_, T>>
        {
            let address = format!("{}:{}", address, port);
            let socket = TcpStream::connect(&address).await.map_err(Error::IO)?;
            Ok(Channel{ socket, ghost: Default::default() })
        }

        pub async fn accept(address: &Ipv4Addr, port: u16)
            -> Result<Channel<T>, Error<'_, T>>
        {
            let address = format!("{}:{}", address, port);
            let mut listener = TcpListener::bind(&address).await.map_err(Error::IO)?;
            let (socket, _) = listener.accept().await.map_err(Error::IO)?;
            Ok(Channel{ socket, ghost: Default::default() })
        }

        pub fn split(&mut self)
            -> (Sender<'_, T>, Receiver<'_, T>)
        {
            let (reader, writer) = self.socket.split();

            let reader: FramedRead<
                ReadHalf,
                LengthDelimitedCodec,
            > = FramedRead::new(reader, LengthDelimitedCodec::new());
            let reader = SymmetricallyFramed::new(reader, SymmetricalBincode::default());

            let writer: FramedWrite<
                WriteHalf,
                LengthDelimitedCodec,
            > = FramedWrite::new(writer, LengthDelimitedCodec::new());
            let writer = SymmetricallyFramed::new(writer, SymmetricalBincode::default());

            (Sender{ writer }, Receiver{ reader })
        }
    }
}

pub mod asymmetric {

    use futures::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::net::Ipv4Addr;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::net::tcp::{ReadHalf, WriteHalf};
    use tokio_serde::Framed;
    use tokio_serde::formats::*;
    use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

    #[derive(Debug)]
    pub enum Error<'a, I, O> where
        I: for<'de> Deserialize<'de> + Serialize,
        O: for<'de> Deserialize<'de> + Serialize,
    {
        IO(std::io::Error),
        ReadError(<Reader<'a, I, O> as futures::TryStream>::Error),
        WriteError(<Writer<'a, I, O> as futures::Sink<I>>::Error),
    }

    pub type Reader<'a, I, O> = Framed<
        FramedRead<ReadHalf<'a>, LengthDelimitedCodec>,
        O,
        I,
        Bincode<O, I>>;

    pub type Writer<'a, I, O> = Framed<
        FramedWrite<WriteHalf<'a>, LengthDelimitedCodec>,
        O,
        I,
        Bincode<O, I>>;

    pub struct Receiver<'a, I, O> {
        reader: Reader<'a, I, O>,
    }

    impl<'a, I, O> Receiver<'a, I, O> where
        I: for<'de> Deserialize<'de> + Serialize,
        O: for<'de> Deserialize<'de> + Serialize,
        Reader<'a, I, O> : TryStream<Ok=O> + Unpin,
    {
        pub async fn recv(&mut self) -> Result<Option<O>, Error<'a, I, O>> {
            Ok(self.reader.try_next().await.map_err(Error::ReadError)?)
        }
    }

    pub struct Sender<'a, I, O> {
        writer: Writer<'a, I, O>,
    }

    impl<'a, I, O> Sender<'a, I, O> where
        I: for<'de> Deserialize<'de> + Serialize,
        O: for<'de> Deserialize<'de> + Serialize,
        Writer<'a, I, O> : Sink<I> + Unpin
    {
        pub async fn send(&mut self, item: I) -> Result<(), Error<'a, I, O>> {
            Ok(self.writer.send(item).await.map_err(Error::WriteError)?)
        }
    }

    pub struct Channel<I, O> {
        socket: TcpStream,
        ghost: std::marker::PhantomData<(I, O)>,
    }

    impl<'a, I, O> Channel<I, O> where
        I: for<'de> Deserialize<'de> + Serialize,
        O: for<'de> Deserialize<'de> + Serialize,
    {
        pub async fn connect(address: &Ipv4Addr, port: u16)
            -> Result<Channel<I, O>, Error<'a, I, O>>
        {
            let address = format!("{}:{}", address, port);
            let socket = TcpStream::connect(&address).await.map_err(Error::IO)?;
            Ok(Channel{ socket, ghost: Default::default() })
        }

        pub async fn accept(address: &Ipv4Addr, port: u16)
            -> Result<Channel<I, O>, Error<'a, I, O>>
        {
            let address = format!("{}:{}", address, port);
            let mut listener = TcpListener::bind(&address).await.map_err(Error::IO)?;
            let (socket, _) = listener.accept().await.map_err(Error::IO)?;
            Ok(Channel{ socket, ghost: Default::default() })
        }

        pub fn split(&mut self)
            -> (Sender<'_, I, O>, Receiver<'_, I, O>)
        {
            let (reader, writer) = self.socket.split();

            let reader: FramedRead<
                ReadHalf,
                LengthDelimitedCodec,
            > = FramedRead::new(reader, LengthDelimitedCodec::new());
            let reader = Framed::new(reader, Bincode::default());

            let writer: FramedWrite<
                WriteHalf,
                LengthDelimitedCodec,
            > = FramedWrite::new(writer, LengthDelimitedCodec::new());
            let writer = Framed::new(writer, Bincode::default());

            (Sender{ writer }, Receiver{ reader })
        }
    }
}
