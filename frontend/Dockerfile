FROM node:14

# Setting working directory. All the path will be relative to WORKDIR
WORKDIR /usr/src/app

# Installing dependencies
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Copying source files
COPY . .

# Building app
RUN yarn test
RUN NEXT_PUBLIC_INSTRUMENTATION_KEY=18be3b83-4ad5-4186-a7b6-f65dd1253a77 yarn build

# Running the app
CMD [ "yarn", "start" ]
