mod business_logic;
mod database;
mod parcing;
mod cli;

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
