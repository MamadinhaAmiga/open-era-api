# **Palms-Open API 🚀**

The **Palms-Open API** is a cutting-edge Rust-based open-source API designed to revolutionize how traders and developers interact with token information. By harnessing the power of **Large Language Models (LLMs)** and **Machine Learning**, the Palms-Open API delivers unparalleled insights, enabling precise token analysis and robust decision-making for Solana traders and developers worldwide.

## **Why Palms-Open API?**

- **LLM-Powered Analysis**: Advanced language models take token analysis to a whole new level, providing sophisticated insights beyond traditional metrics.
- **Machine Learning Integration**: Stay ahead of market trends with real-time predictions powered by ML algorithms tuned for the dynamic nature of cryptocurrency markets.
- **Rust Performance**: Built in Rust for unparalleled performance, security, and scalability.
- **Decentralized Ready**: Designed to seamlessly integrate into decentralized environments and empower autonomous trading systems.

The **Palms-Open API** isn’t just an API—it’s the future of token intelligence.

---

## **Features**

- Token data aggregation and enhanced analysis using state-of-the-art AI techniques.
- Flexible endpoints for retrieving deep market insights.
- Optimized for Solana, but adaptable to other blockchain ecosystems.
- Ready-to-integrate into decentralized trading agents like Palms.

---

## **Build Instructions**

Follow these steps to set up the **Palms-Open API** locally:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-org/palms-open-api.git
   cd palms-open-api
   ```

2. **Install Rust**:
   Ensure you have Rust installed. If not, download it [here](https://www.rust-lang.org/tools/install).

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

3. **Install Dependencies**:
   Use `cargo` to fetch and install dependencies:
   ```bash
   cargo build --release --target=x86_64-unknown-linux-musl
   ```
   --release and --target flag is needed to build binary accordingly to lambdas environment 

4. **Run Tests**:
   Verify the setup by running tests:
   ```bash
   cargo test
   ```

---

## **Run Locally**

1. **Start the API**:
   ```bash
   cargo run --release
   or
   sam local start-api --env-vars env.json
   ```

2. **Access the API**:
   By default, the API runs on `http://127.0.0.1:3000`. You can interact with the endpoints using tools like `curl`, Postman, or connect your webapp.

3. **Example Request**:
   ```bash
   curl -X GET http://127.0.0.1:3000/tokeninfo?token_id=HeLp6NuQkmYB4pYWo2zYs22mESHXPQYzXbB8n4V98jwC
   ```

---

## **Next Steps on the Roadmap**

### 🔥 **Phase 1: Advanced Token Analysis**
Refine ML models and LLMs for hyper-accurate market predictions and enhanced token profiling.

### 🌟 **Phase 2: Blockchain Interoperability**
Expand support beyond Solana to include Ethereum, Polygon, and other leading chains.

### 🚀 **Phase 3: Decentralized Intelligence**
Deploy the API in a fully decentralized manner, enabling trustless and unstoppable token analysis solutions.

### 🌐 **Phase 4: Ecosystem Expansion**
Integrate Palms-Open API with trading platforms, bots, and decentralized applications (DApps) for maximum adoption.

---

## **Contributing**

We welcome contributions from the community! Feel free to fork the repo, make your changes, and submit a pull request. Let’s shape the future of token intelligence together!

---

## **License**

This project is licensed under the [MIT License](LICENSE).
