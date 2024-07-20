# get-command-line

This project provides a Node.js addon written in Rust to retrieve the command line of a process by its name on Windows.

## Prerequisites

- [Node.js](https://nodejs.org/) (>= 18.x)
- [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/)

## Installation

1. Install the package:
    ```sh
    yarn add @tipspace/get-command-line
    # or
    npm install @tipspace/get-command-line
    ```

## Usage

### JavaScript

```javascript
const { getProcessCommandLine } = require('@tipspace/get-command-line');

try {
  const commandLine = getProcessCommandLine('process_name');
  console.log(`Command Line: ${commandLine}`);
} catch (err) {
  console.error(`Error: ${err.message}`);
}
```

### Typescript

```typescript
import { getProcessCommandLine } from '@tipspace/get-command-line';

try {
  const commandLine = getProcessCommandLine('process_name');
  console.log(`Command Line: ${commandLine}`);
} catch (err) {
  console.error(`Error: ${err.message}`);
}
```

## Development

### Building

You'll need [Rust](https://www.rust-lang.org/) (>= 1.50.0) to build the project.
To build the project, run:

```
yarn build
# or
npm run build
```

### Testing

To run test, use:

```
yarn test
# or
npm test
```

## Contributing

1. Fork the repository.
2. Create a new branch (git checkout -b feature-branch).
3. Make your changes.
4. Commit your changes (git commit -am 'Add new feature').
5. Push to the branch (git push origin feature-branch).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
