FROM node:18

RUN apt-get update
# install dependencies for rust and tauri
RUN apt-get install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# install rust
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo -V

WORKDIR /usr/src/biene
# copy data to working dir
COPY . .
RUN npm install
CMD ["npm", "run", "tauri", "dev"]
