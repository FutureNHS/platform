This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/zeit/next.js/tree/canary/packages/create-next-app).

## Getting Started
Hello :-)

## How to use

Frontend development server relies on some services that are available in the development cluster. Specifically, it calls out to the hello-world service when doing server-side rendering, and also proxies `/hello/*` to `http://hello-world.hello-world/hello/*` like how our nginx ingress does in production.

In order to set up a development cluster, see ../infrastructure/README.md. Once your development cluster is set up, you should run this to make the cluster's services available for local development:

```bash
sudo kubefwd services -n hello-world -n frontend -n argocd
```

Once that is running, start the dev server like this:

```bash
yarn dev
```

Open [http://localhost:4455](http://localhost:4455) with your browser to see the result.The port is currently set to 4455 to access [Kratos](https://www.ory.sh/kratos/docs/quickstart)

You can start editing the page by modifying `pages/index.js`. The page auto-updates as you edit the file.

## Other useful things to know

### Code formatting

We use [Prettier](https://prettier.io/) to format our code ((js/ts, tsx/jsx, md, yaml, graphql, html). Prettier will run in a Github action to verify code that is pushed. CI will fail if it detects formatting errors.

If you use VSCode, you may find it useful to install [this extension](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode), and (optionally) set your editor to format 'on save'.
