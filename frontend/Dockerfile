# Use official Node.js image as base
FROM node:20.11.0

# Set working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application
COPY . .

# Expose the port
EXPOSE 3000

# Command to run the app
CMD ["npm", "run", "dev"]