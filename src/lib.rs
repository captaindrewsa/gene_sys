pub mod business_logic;
pub mod database;
pub mod parcing;
pub mod cli;

pub mod interfaces{
    pub trait IDatabase{
        //!Трейт для общего взаимодейтсвия с базой данных

    }

    pub trait IParcing {
        //!Трейт для общего взаимодейтсвия с парсером
    }

    pub trait ILogic {
        //!Трейт для общего взаимодейтсвия с логикой
    }

    pub trait ICLI {
        //!Трейт для общего взаимодейтсвия с интерфейсом      
    }
}


/**
Общий enum Ошибок 
*/
#[non_exhaustive]
pub enum Errors {
    /**
    Определение ошибки 
    */
    UndefindError
}
