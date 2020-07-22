// html form
// cross site request forgery CSRF
// on submit sends a POST to ory
// next.js routing?
// onSubmit there's a request

// if no requestId then forward to /.ory/kratos/public/self-service/browser/flows/login => forwards back to auth/login/?request=1234 my own html form with a requestId
// use getServersideProps context, check for requestId in params and if not exist then forward res.redirect
// if requestId render html form with username and password

// Home -> Login -> Login ServerSideProps runs -> No request id -> redirect Kratos -> redirects Login (done by kratos) -> serverSideProps runs -> requestId present

// Home -> Login -> ServerSideProps runs -> request id present
