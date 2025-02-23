use crate::internal::model::auth;

pub fn auth_handler<T>(auth_service: T)
where
    T: auth::Service,
{
    
}
