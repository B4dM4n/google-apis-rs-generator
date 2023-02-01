use hyper::{client::connect::Connection, service::Service, Uri};
use tokio::io::{AsyncRead, AsyncWrite};
use yup_oauth2::authenticator::Authenticator;

pub fn from_authenticator<C, I, S>(auth: Authenticator<C>, scopes: I) -> impl crate::GetAccessToken
where
    C: Service<Uri> + Clone + Send + Sync + 'static,
    C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    C::Future: Send + Unpin + 'static,
    C::Error: Into<Box<dyn ::std::error::Error + Send + Sync>>,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    YupAuthenticator {
        auth,
        scopes: scopes.into_iter().map(Into::into).collect(),
    }
}

struct YupAuthenticator<C> {
    auth: Authenticator<C>,
    scopes: Vec<String>,
}

impl<T> ::std::fmt::Debug for YupAuthenticator<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "YupAuthenticator{{..}}")
    }
}

#[async_trait::async_trait]
impl<C> crate::GetAccessToken for YupAuthenticator<C>
where
    C: Service<Uri> + Clone + Send + Sync + 'static,
    C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    C::Future: Send + Unpin + 'static,
    C::Error: Into<Box<dyn ::std::error::Error + Send + Sync>>,
{
    async fn access_token(&self) -> Result<String, Box<dyn ::std::error::Error + Send + Sync>> {
        Ok(self
            .auth
            .token(&self.scopes)
            .await?
            .token()
            .ok_or("authenticator did not produce an access_token")?
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GetAccessToken;
    use yup_oauth2 as oauth2;

    #[tokio::test]
    async fn it_works() {
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            oauth2::ApplicationSecret::default(),
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .build()
        .await
        .expect("failed to build");

        let auth = from_authenticator(auth, vec!["foo", "bar"]);

        fn this_should_work<T: GetAccessToken>(_x: T) {}
        this_should_work(auth);
    }
}
