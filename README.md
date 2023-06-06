# Simple REST API with Rust & Rocket Framework
<br>
<p>This GitHub repository focuses on developing a simple RESTful API using Rust and the Rocket framework. It demonstrates CRUD operations through API endpoints for creating, reading, updating, and deleting data. Specific code files have been uploaded for sharing code references, enabling others to understand implementations. The codebase is being improved for consistency in naming conventions, code structure, formatting, and error handling.</p>
<br>
<h3>Setup</h3>
<p>1. <a href="https://www.rust-lang.org/tools/install" target="_blank">Rust</a> (Version: rustc 1.69.0-nightly)</p>
<p>1. Rocket Framework (Version: 0.5.0-rc.3)</a></p>
<p>3. <a href="https://www.apachefriends.org/download.html" target="_blank">XAMPP</a> (MySQL Server)</p>
<p>3. <a href="https://code.visualstudio.com/" target="_blank">Visual Studio Code</a></p>
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
