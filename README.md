# The Good Shop

This is a charity platform dedicated to helping people in Gaza through donations and aid.

## Architecture Overview

The Good Shop is built using a modern stack with the following components:

### Frontend
- **Framework**: [Vike](https://vike.dev) with [React](https://react.dev)
- **UI**: TailwindCSS with DaisyUI
- **Payment**: Stripe integration via Stripe.js and React Stripe.js for accepting donations
- **Routing**: Vike's built-in routing system

### Backend
- **Framework**: Actix-web (Rust)
- **Database**: SQLite with SQLx for type-safe queries
- **Payment Processing**: Stripe API integration for donation processing
- **Type Sharing**: Specta for generating TypeScript types from Rust types

### Directory Structure

```
thegoodshop/                # Frontend application
├── components/             # Reusable UI components
├── database/               # Frontend data management
├── layouts/                # Page layouts
├── pages/                  # Application pages and routes
├── renderer/               # Vike rendering configuration
└── src/                    # Frontend source code

src/                        # Backend Rust code
├── main.rs                 # Server entrypoint and API routes
├── db.rs                   # Database connection and queries
├── model.rs                # Data models shared between frontend and backend
└── lib.rs                  # Library exports

migrations/                 # SQLite database migrations
```

## Key Features

1. **Type Safety**: Full stack type safety with TypeScript on the frontend and Rust on the backend
2. **Server-Side Rendering**: SSR enabled by default
3. **Donation Processing**: Stripe integration for secure donation processing
4. **Data Management**: Aid programs and donations stored in SQLite database

## Development Workflow

1. The Rust backend serves the API endpoints and static frontend files
2. Frontend is built with Vike for optimized production builds
3. Types are automatically shared between frontend and backend using Specta

## API Endpoints

- `GET /api/products` - Retrieve all aid programs
- `POST /api/create-checkout-session` - Create a Stripe checkout session for donations

## Database Schema

The application uses a SQLite database with the following schema:

```sql
CREATE TABLE products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (...)
);
```

Note: While the table is named "products", these entries represent aid programs for Gaza that users can donate to.

## Getting Started With Nix

This project uses [Nix](https://nixos.org/) with [direnv](https://direnv.net/) for dependency management, making setup extremely simple.

### Prerequisites

- [Nix](https://nixos.org/download.html) package manager
- [direnv](https://direnv.net/docs/installation.html) for environment management

### Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd thegoodshop
   ```

2. Allow direnv to set up the environment:
   ```bash
   direnv allow
   ```
   This single command will:
   - Install all required dependencies (Rust, Node.js, pnpm, SQLite, etc.)
   - Set up the development environment
   - Configure necessary environment variables

3. Start the application:
   ```bash
   # Build and run the backend
   cargo run
   ```

4. The application will be available at `http://localhost:5526`

### Stripe Setup

For payment processing, you'll need to configure your Stripe API key:

1. Edit the `.envrc` file to add your Stripe API key:
   ```
   DATABASE_URL="sqlite://./db/shop.db"
   export DATABASE_URL
   export STRIPE_API_KEY="sk_test_your_stripe_secret_key"
   
   use flake
   ```

2. Run `direnv allow` again to apply the changes.

## Getting Started Without Nix

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v16+)
- [pnpm](https://pnpm.io/installation) (v8+)
- [SQLite](https://www.sqlite.org/download.html)
- A [Stripe](https://stripe.com) account with API keys

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd thegoodshop
   ```

2. Install frontend dependencies:
   ```bash
   cd thegoodshop
   pnpm install
   ```

3. Install Rust dependencies:
   ```bash
   # From the project root
   cargo build
   ```

### Environment Setup

1. Create a `.env` file in the project root:
   ```
   DATABASE_URL=sqlite://db/mydb.sqlite
   STRIPE_API_KEY=sk_test_your_stripe_secret_key
   ```

2. Replace `sk_test_your_stripe_secret_key` with your actual Stripe secret key.

### Database Setup

The database will be automatically set up when you first run the application, with migrations applied to create the necessary tables.

### Development Mode

1. Start the development server:
   ```bash
   # From the project root
   cargo run
   ```

2. The application will be available at `http://localhost:5526`

### Building for Production

1. Build the frontend:
   ```bash
   cd thegoodshop
   pnpm build
   ```

2. Build the Rust backend:
   ```bash
   # From the project root
   cargo build --release
   ```

3. Run the production server:
   ```bash
   ./target/release/thegoodshop
   ```

### Adding New Aid Programs

To add new aid programs to the platform:

1. You can manually insert entries into the SQLite database:
   ```sql
   INSERT INTO products (name, price) VALUES ('Emergency Food Package', 25.00);
   INSERT INTO products (name, price) VALUES ('Medical Supplies Kit', 50.00);
   ```

2. Or modify the initial migration file at `migrations/20250318163726_Initial_Migration.sql` to include your desired aid programs.

### Customizing the Frontend

1. Main page content: Edit `thegoodshop/pages/index/+Page.tsx`
2. Checkout process: Modify files in `thegoodshop/pages/checkout/`
3. Styles: Update Tailwind configuration in `thegoodshop/tailwind.config.js`

### Deployment

For production deployment:

1. Set up a server with Rust installed
2. Configure a reverse proxy (like Nginx) in front of the application
3. Set up environment variables with production values
4. Use a process manager like systemd or supervisor to keep the application running

For more information on Vike, see the [Vike documentation](https://vike.dev).

