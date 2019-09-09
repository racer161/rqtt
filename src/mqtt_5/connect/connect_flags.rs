use super::qos::Qos;

struct ConnectFlags
{
    UserNameFlag : bool,
    PasswordFlag : bool,
    WillRetain : bool,
    WillQos : Qos,
    WillFlag : bool,
    CleanStart: bool,
    Reserved : bool,
}