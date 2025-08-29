# RAG System Web Interface

Modern React-based web interface for the AI-Rust-API RAG system, providing comprehensive user and admin functionality with Material-UI design.

## 🚀 Features

### 🔐 User Authentication
- **Login/Registration**: Secure user authentication
- **Role-Based Access**: Different interfaces for admin and regular users
- **JWT Integration**: Seamless token-based authentication
- **Responsive Design**: Works on desktop and mobile devices

### 🎨 User Dashboard
- **Interactive RAG Chat**: Natural language Q&A interface
- **RAG Model Selection**: Choose specific RAG models for queries
- **Source Attribution**: View document sources for each answer
- **Chat History**: Keep track of conversation history
- **Real-Time Responses**: Live streaming of AI responses

### ⚙️ Admin Dashboard
- **System Overview**: Real-time system statistics and health
- **User Management**: Create, update, and manage user accounts
- **Document Management**: Upload, organize, and manage documents
- **Vector Management**: Create and manage document vectors
- **RAG Model Management**: Configure RAG models with custom prompts
- **Environment Configuration**: Secure system configuration management
- **System Analytics**: Comprehensive system monitoring

## 📦 Available Scripts

### `npm start`
Runs the app in development mode at [http://localhost:3000](http://localhost:3000)

### `npm run build`
Builds the app for production to the `build` folder with optimized performance

### `npm test`
Launches the test runner in interactive watch mode

### `npm run eject`
**Note: This is a one-way operation!** Removes the single build dependency and gives full control over configuration

## 🎯 User Interface Components

### Authentication Flow
- **Login Page**: Clean, modern login interface
- **Registration Page**: User-friendly registration form
- **Protected Routes**: Automatic redirection for unauthenticated users

### User Experience
- **UserDashboard**: Main interface for regular users
  - RAG chat interface with model selection
  - Source citation and document references
  - Conversation history and context
  
- **AdminDashboard**: Comprehensive admin interface with tabs:
  - **Overview**: System statistics and health monitoring
  - **User Management**: User creation, editing, and role management
  - **Configuration**: Environment variable management
  - **Documents**: File upload and folder organization
  - **Vectors**: Vector creation and processing management
  - **RAG Models**: Model creation with custom system prompts

## 🏗️ Architecture

```
useragent/
├── public/
│   ├── index.html           # Main HTML template
│   ├── favicon.ico         # Application icon
│   └── manifest.json       # PWA manifest
├── src/
│   ├── components/         # React components
│   │   ├── Login.js        # Authentication component
│   │   ├── Register.js     # User registration
│   │   ├── UserDashboard.js # Main user interface
│   │   ├── AdminDashboard.js # Admin navigation
│   │   └── admin/          # Admin-specific components
│   │       ├── AdminDashboard.js # System overview
│   │       ├── UserManagement.js # User admin tools
│   │       ├── DocumentManagement.js # File management
│   │       ├── VectorManagement.js # Vector tools
│   │       ├── RagModelManagement.js # RAG model tools
│   │       └── EnvConfigurationManagement.js # Config tools
│   ├── context/            # React contexts
│   │   └── AuthContext.js  # Authentication state management
│   ├── services/           # API integration
│   │   ├── authService.js  # Authentication API calls
│   │   └── adminService.js # Admin API calls
│   ├── App.js              # Main application component
│   ├── App.css             # Application styles
│   └── index.js            # Application entry point
├── package.json            # Dependencies and scripts
└── README.md               # This documentation
```

## 🔧 Configuration

### Environment Variables
Create a `.env` file in the useragent directory:

```bash
# API Endpoints
REACT_APP_AUTH_API_URL=http://localhost:9102
REACT_APP_CONFIG_API_URL=http://localhost:9103
REACT_APP_RAG_API_URL=http://localhost:9101

# Application Settings
REACT_APP_APP_NAME=RAG System
REACT_APP_VERSION=1.0.0
```

### Default Credentials
- **Admin Username**: admin
- **Admin Password**: password
- **Admin Email**: admin@example.com

## 🚀 Getting Started

### Prerequisites
- Node.js 14+ and npm
- Backend services running (AuthAPI, UIConfigAPI, RAGAPI)

### Installation & Setup
```bash
# Navigate to frontend directory
cd useragent

# Install dependencies
npm install

# Start development server
npm start

# Open browser to http://localhost:3000
```

### Production Build
```bash
# Create optimized production build
npm run build

# Serve static files (example with serve)
npx serve -s build -l 3000
```

## 🎨 User Interface Features

### Material-UI Integration
- **Modern Design**: Clean, professional interface using Material-UI
- **Responsive Layout**: Adapts to different screen sizes
- **Consistent Theming**: Unified color scheme and typography
- **Accessibility**: WCAG-compliant design patterns

### Interactive Components
- **Real-Time Chat**: Live RAG conversation interface
- **File Uploads**: Drag-and-drop document uploads
- **Data Tables**: Sortable, filterable data displays
- **Form Validation**: Client-side validation with error messages
- **Progress Indicators**: Visual feedback for long-running operations

### Admin Tools
- **System Monitoring**: Real-time health and statistics
- **Bulk Operations**: Efficient management of multiple items
- **Advanced Search**: Filter and search across all data
- **Export Functions**: Data export capabilities
- **Audit Logging**: Track administrative actions

## 🔗 API Integration

### Authentication Flow
1. User enters credentials on login page
2. Frontend calls AuthAPI for token generation
3. Token stored in local storage and context
4. All subsequent API calls include Bearer token
5. Automatic token refresh on expiration

### Service Communication
- **AuthAPI**: User authentication and token validation
- **UIConfigAPI**: All admin functionality and user management
- **RAGAPI**: RAG queries and document processing

## 🛠️ Key Dependencies

### Core Framework
- **React 18**: Modern React with hooks and concurrent features
- **React Router**: Client-side routing and navigation

### UI Components
- **@mui/material**: Material-UI component library
- **@mui/icons-material**: Material-UI icons
- **@emotion/react**: CSS-in-JS styling

### Development Tools
- **Create React App**: Development environment and build tools
- **Web Vitals**: Performance monitoring
- **Testing Library**: Component testing utilities

## 🚨 Security Features

- **JWT Token Management**: Secure token storage and refresh
- **Protected Routes**: Authentication-required pages
- **Role-Based UI**: Different interfaces for admin vs users
- **Input Validation**: Client-side form validation
- **XSS Protection**: Sanitized user inputs

## 📱 Responsive Design

- **Mobile-First**: Optimized for mobile devices
- **Tablet Support**: Adaptive layout for tablets
- **Desktop Enhancement**: Full features on desktop
- **Touch-Friendly**: Appropriate touch targets and gestures

## 🔍 Advanced Features

### RAG Chat Interface
- **Model Selection**: Choose between different RAG models
- **Source Attribution**: View document sources for answers
- **Context Awareness**: Maintains conversation context
- **Export Conversations**: Save chat history

### Admin Dashboard
- **Real-Time Updates**: Live system statistics
- **Batch Operations**: Efficient bulk management
- **Advanced Filtering**: Complex search and filter options
- **Data Visualization**: Charts and graphs for system metrics

## 📊 Performance Optimizations

- **Code Splitting**: Lazy-loaded components for faster initial load
- **Bundle Optimization**: Minimized JavaScript bundles
- **Asset Optimization**: Compressed images and assets
- **Caching Strategy**: Efficient browser caching
- **Service Worker**: PWA capabilities for offline functionality

This web interface provides a complete, user-friendly way to interact with the RAG system, from simple Q&A for end users to comprehensive system administration for admins.
