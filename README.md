# Simple REST API with Rust & Rocket Framework
<br>
<h3>SETUP</h3>
<p>1. <a href="https://www.rust-lang.org/tools/install" target="_blank">Rust</a></p>
<p>3. <a href="https://www.apachefriends.org/download.html" target="_blank">XAMPP</a> (MySQL Server)</p>
<p>3. <a href="https://code.visualstudio.com/" target="_blank">Visual Studio Code</a></p>
<br>
<h3>HOW TO RUN</h3>
<p>1. Download folder</p>
<p>2. Import database to MySQL</p>
<p>3. Open folder from VS Code</p>
<p>4. Setup database pool connection in Rocket.toml (If needed)</p>
<p>5. Open new terminal (Pointing to folder directory) and start Rocket server with this command: <code>cargo run</code></p>
<p>6. By default, Rocket server runs on Port 8000</code></p>
<p>7. Test API using <code>cURL</code> or tools like Thunder Client/Postman</p>
<br>
<h3>API Endpoints (localhost)</h3>
<p>Fetch all data: <code>http://127.0.0.1/all</code></p>
<p>Fetch data by ID: <code>http://127.0.0.1/ID</code></p>
<p>Insert data: <code>http://127.0.0.1/insert</code></p>
<p>Update data by ID: <code>http://127.0.0.1/update/ID</code></p>
<p>Update data by ID: <code>http://127.0.0.1/delete/ID</code></p>
<br> 
<h3>Notes</h3>
<ul>
  <li><p>Insert/Update data can be from local or remote (http/https) JSON files and raw JSON data.</p>
    <p>Example of JSON data: <code>{"code":"P999","name":"Soccer Ball","price":19.99}"</code></p>
  </li>
  <li><p>All Rust dependencies/crates can be obtained from <a href="https://crates.io/" target="_blank">crates.io</a></p></li>
</ul>
