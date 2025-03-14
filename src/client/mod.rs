pub const HTTP_CLIENT: &str = r#"
<!-- Fluxor: Simple HTTP Client -->
<!-- Author: Montasir Mirghani <me@montasir.site> 2024-->
<!-- Send message as json body (method => post)-->
<!-- https://httpbin.org/post -->
<!-- Retrieve users collection (method => get) -->
<!-- https://jsonplaceholder.typicode.com/users -->
<!DOCTYPE html>
<html lang="en" x-data="httpClient()" x-init="init()"
  :class="darkMode ? 'bg-gray-800 text-white' : 'bg-gray-100 text-black'">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Simple HTTP Client</title>
  <script src="https://cdn.tailwindcss.com/3.4.16"></script>
  <script src="https://cdn.jsdelivr.net/npm/alpinejs@3.14.8/dist/cdn.min.js" defer></script>
  <style>
    /* Add custom styles for the response area */
    .response-box {
      max-height: 300px;
      overflow-x: auto;
      overflow-y: auto;
    }

    .loader {
      display: none;
      /* Initially hidden */
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      z-index: 999;
    }
  </style>
</head>

<body class="p-6">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold mb-4">Fluxor <small>🔥 Simple HTTP Client 🔥</small></h1>
    <button @click="toggleDarkMode"
      :class="darkMode ? 'bg-gray-600 hover:bg-gray-500' : 'bg-gray-300 hover:bg-gray-400'"
      class="mb-5 font-bold dark:bg-gray-700 p-2 rounded" title="Toggle Dark/Light Mode">
      <span x-text="darkMode ? 'Light Mode' : 'Dark Mode'"></span>
    </button>
  </div>

  <form @submit.prevent="sendRequest" :class="darkMode ? 'bg-gray-900' : 'bg-gray-200'"
    class="bg-white dark:bg-gray-900 p-6 rounded-lg shadow-md">
    <div class="mb-4">
      <input type="text" x-model="url" placeholder="Enter URL" required
        :class="darkMode ? 'bg-gray-800 text-white' : 'bg-white'"
        class="border dark:border-gray-600 border-gray-300 p-2 w-full rounded">
    </div>
    <div class="mb-4">
      <select x-model="method" :class="darkMode ? 'bg-gray-800 text-white' : 'bg-white'"
        class="font-bold border dark:border-gray-600 border-gray-300 p-2 w-full rounded">
        <option value="GET">GET</option>
        <option value="POST">POST</option>
        <option value="PUT">PUT</option>
        <option value="PATCH">PATCH</option>
        <option value="DELETE">DELETE</option>
      </select>
    </div>
    <div class="mb-4">
      <textarea x-model="headers" placeholder="Headers (JSON format)"
        :class="darkMode ? 'bg-gray-800 text-white' : 'bg-white'"
        class="border dark:border-gray-600 border-gray-300 p-2 w-full rounded"></textarea>
    </div>
    <div class="mb-4">
      <input type="file" @change="handleFileChange" multiple :class="darkMode ? 'bg-gray-800 text-white' : 'bg-white'"
        class="border dark:border-gray-600 border-gray-300 p-2 w-full rounded">
    </div>
    <div class="mb-4" x-show="method !== 'GET' && method !== 'DELETE'">
      <textarea rows='5' x-model="body" placeholder="Body (JSON format, if applicable)"
        :class="darkMode ? 'bg-gray-800 text-white' : 'bg-white'"
        class="border dark:border-gray-600 border-gray-300 p-2 w-full rounded"></textarea>
    </div>
    <button type="submit"
      class="mt-2 font-bold bg-gray-600 text-[#61dafb] p-2 rounded hover:text-black hover:bg-[#61dafb]">Send
      Request</button>
  </form>

  <!-- Spinner Element -->
  <div id="loader" class="loader" role="status">
    <div class="w-8 h-8 border-8 border-dashed rounded-full animate-spin border-[#61dafb]"></div>
    <span class="sr-only">Loading...</span>
  </div>

  <div class="mt-6">
    <h2 class="text-xl font-semibold pb-4">Response:</h2>
    <div class="relative">
      <button @click="copyResponse"
        :class="darkMode ? 'bg-gray-700 hover:bg-gray-600' : 'bg-gray-300 hover:bg-gray-400'"
        class="absolute top-2 left-2 pl-1 p-2 rounded" title="Copy">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M8 6h12a2 2 0 012 2v12a2 2 0 01-2 2H8a2 2 0 01-2-2V8a2 2 0 012-2z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M8 6V4a2 2 0 012-2h8a2 2 0 012 2v2M12 12v4m0-4h4m-4 0H8" />
        </svg>
      </button>
      <pre x-text="response" :class="darkMode ? 'bg-gray-900' : 'bg-gray-200'" class="response-box p-10 rounded"></pre>
      <div x-show="copySuccess"
        class="font-bold absolute top-2 left-12 bg-green-500 text-white p-1 rounded transition-opacity duration-300">
        Response copied to clipboard!
      </div>
    </div>
  </div>

  <footer class="my-7" x-data="{ year: new Date().getFullYear() }">
    <div class="ml-7 mb-5 w-28 h-28">
      <a href='#'>
        <svg viewBox="0 0 66 82" fill="none" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%">
          <path d="M57.8291 77.1871V75.122H60.2276C60.6611 75.122 61.0418 75.0403 61.3698 74.877C61.6978 74.7137 61.9526 74.4861 62.1342 74.1944C62.3216 73.8969 62.4153 73.5527 62.4153 73.1619C62.4153 72.5552 62.2045 72.0797 61.7827 71.7355C61.361 71.3855 60.8075 71.2105 60.1222 71.2105H57.8291V68.9616H60.5176C61.3434 68.9616 62.0844 69.1133 62.7404 69.4166C63.4023 69.72 63.9265 70.1604 64.3131 70.738C64.6997 71.3155 64.893 72.0214 64.893 72.8556C64.893 73.6665 64.6938 74.3986 64.2955 75.052C63.8972 75.7054 63.3261 76.2246 62.5823 76.6096C61.8384 76.9946 60.951 77.1871 59.9201 77.1871H57.8291ZM56.0719 81.8599V68.9616H58.5847V81.8599H56.0719ZM63.1534 81.8599L60.1749 76.8284L62.0639 75.3408L66 81.8599H63.1534Z" :fill="darkMode ? '#e0e0e0' : '#5e5e5e'"/>
          <path d="M47.7077 82C46.7822 82 45.9183 81.8338 45.1158 81.5012C44.3192 81.1687 43.6193 80.702 43.016 80.1011C42.4127 79.5003 41.9412 78.8002 41.6014 78.001C41.2676 77.2018 41.1006 76.3355 41.1006 75.4021C41.1006 74.4745 41.2676 73.6111 41.6014 72.8119C41.9412 72.0127 42.4097 71.3156 43.0072 70.7205C43.6105 70.1255 44.3104 69.6617 45.107 69.3292C45.9095 68.9908 46.7705 68.8217 47.6901 68.8217C48.6273 68.8217 49.4941 68.9908 50.2907 69.3292C51.0932 69.6617 51.7931 70.1255 52.3906 70.7205C52.9939 71.3156 53.4595 72.0127 53.7875 72.8119C54.1214 73.6111 54.2883 74.4774 54.2883 75.4108C54.2883 76.3384 54.1214 77.2018 53.7875 78.001C53.4595 78.8002 52.9939 79.5003 52.3906 80.1011C51.7931 80.702 51.0961 81.1687 50.2995 81.5012C49.5029 81.8338 48.639 82 47.7077 82ZM47.6813 79.7161C48.267 79.7161 48.8059 79.6053 49.2979 79.3836C49.7958 79.1619 50.2292 78.8556 50.5982 78.4648C50.9731 78.0739 51.263 77.6189 51.4681 77.0997C51.6731 76.5747 51.7756 76.0088 51.7756 75.4021C51.7756 74.7837 51.6701 74.2149 51.4593 73.6957C51.2543 73.1707 50.9643 72.7157 50.5895 72.3306C50.2204 71.9456 49.787 71.6452 49.2891 71.4293C48.7913 71.2135 48.2553 71.1056 47.6813 71.1056C47.1132 71.1056 46.5831 71.2135 46.0911 71.4293C45.599 71.6452 45.1685 71.9456 44.7995 72.3306C44.4305 72.7157 44.1406 73.1707 43.9297 73.6957C43.7189 74.2149 43.6134 74.7837 43.6134 75.4021C43.6134 76.0088 43.7159 76.5747 43.9209 77.0997C44.1318 77.6247 44.4188 78.0827 44.7819 78.4735C45.151 78.8644 45.5815 79.1707 46.0735 79.3923C46.5714 79.6082 47.1073 79.7161 47.6813 79.7161Z" :fill="darkMode ? '#e0e0e0' : '#5e5e5e'" />
          <path d="M37.7005 81.8599L34.7572 77.2834H34.6693L33.0264 75.4195V75.367L28.8442 68.9616H31.7348L34.7045 73.7744H34.7923L36.4529 75.3845V75.437L40.7053 81.8599H37.7005ZM28.774 81.8599L33.0264 75.437V75.3845L34.6693 73.7744H34.7572L37.718 68.9616H40.635L36.4529 75.367V75.4195L34.7923 77.2834H34.7045L31.7524 81.8599H28.774Z" :fill="darkMode ? '#e0e0e0' : '#5e5e5e'" />
          <path d="M22.3427 82C21.3352 82 20.4566 81.7929 19.7069 81.3787C18.963 80.9645 18.386 80.384 17.976 79.6373C17.566 78.8848 17.361 78.0039 17.361 76.9946V68.9616H19.8738V77.1696C19.8738 77.6947 19.9763 78.1526 20.1813 78.5435C20.3922 78.9285 20.6821 79.226 21.0511 79.436C21.426 79.6461 21.8565 79.7511 22.3427 79.7511C23.1041 79.7511 23.7074 79.5148 24.1526 79.0423C24.5977 78.5697 24.8203 77.9455 24.8203 77.1696V68.9616H27.3331V76.9946C27.3331 78.0039 27.1281 78.8848 26.7181 79.6373C26.3139 80.384 25.7399 80.9645 24.996 81.3787C24.2521 81.7929 23.3677 82 22.3427 82Z" :fill="darkMode ? '#e0e0e0' : '#5e5e5e'" />
          <path d="M9.10223 81.8599V68.9616H11.615V79.6111H16.5088V81.8599H9.10223Z" :fill="darkMode ? '#e0e0e0' : '#5e5e5e'" />
          <path d="M0 81.8599V68.9616H7.38019V71.2105H2.51278V74.562H6.59824V76.8109H2.51278V81.8599H0Z" :fill="darkMode ? '#e0e0e0' : '#5e5e5e'" />
          <path fill-rule="evenodd" clip-rule="evenodd" d="M25.4152 6.69989L7.71726 24.3521C3.52827 28.5302 3.52827 35.3044 7.71726 39.4825L25.4152 57.1347C29.6042 61.3128 36.3958 61.3128 40.5848 57.1347L58.2827 39.4825C62.4717 35.3044 62.4717 28.5302 58.2827 24.3521L40.5848 6.69989C36.3958 2.52174 29.6042 2.52174 25.4152 6.69989ZM5.18898 21.8303C-0.396327 27.4012 -0.396327 36.4334 5.18898 42.0042L22.8869 59.6564C28.4722 65.2273 37.5278 65.2273 43.1131 59.6564L60.811 42.0042C66.3963 36.4334 66.3963 27.4012 60.811 21.8303L43.1131 4.17816C37.5278 -1.39272 28.4722 -1.39272 22.8869 4.17815L5.18898 21.8303Z" fill='none' />
          <path d="M33.4712 55.9234L25.0436 47.5176L48.6408 23.9814C53.2952 28.6238 53.2952 36.1506 48.6408 40.793L33.4712 55.9234Z" fill='#61DAFB' />
          <path d="M33.2667 35.5208L23.1536 45.6078L19.7826 42.2455L29.8957 32.1585L33.2667 35.5208Z" fill='#FF6D00' />
          <path d="M33.0023 8.38323L41.4299 16.789L17.8327 40.3253C13.1782 35.6829 13.1782 28.1561 17.8327 23.5137L33.0023 8.38323Z" fill='#FF6D00' />
          <path d="M33.2419 28.8208L43.355 18.7339L46.726 22.0962L36.6129 32.1831L33.2419 28.8208Z" fill='#61DAFB' />
        </svg>
      </a>
    </div>
    <p class="text-gray-600">© <span x-text="year"></span> <a class="underline" href="https://www.fluxor.one" target="_blank">fluxor.one</a> | <a class="underline" href="https://github.com/dr-montasir" target="_blank">Dr. Montasir</a>. All rights reserved.</p>
  </footer>

  <script>
    function httpClient() {
      return {
        url: '',
        method: 'GET',
        headers: '',
        body: '',
        files: [],
        response: '',
        copySuccess: false,
        darkMode: false,
        loading: false,
        timeoutId: null,

        init() {
          this.darkMode = JSON.parse(localStorage.getItem('darkMode')) || false;
        },

        toggleDarkMode() {
          this.darkMode = !this.darkMode;
          localStorage.setItem('darkMode', JSON.stringify(this.darkMode));
        },

        handleFileChange(event) {
          this.files = Array.from(event.target.files);
        },

        async sendRequest() {
          const loader = document.getElementById("loader");
          loader.style.display = "block"; // Show spinner
          this.loading = true; // Set loading state
          this.response = ''; // Clear previous response

          this.timeoutId = setTimeout(() => {
            this.loading = false;
            loader.style.display = "none"; // Hide spinner
            this.response = 'Error: Request timed out after 3 minutes.';
          }, 180000); // Timeout after 3 minutes

          try {
            const options = {
              method: this.method,
              headers: {
                'Content-Type': this.method === 'POST' || this.method === 'PUT' || this.method === 'PATCH' ? 'application/json' : undefined,
                ...this.parseHeaders(this.headers)
              }
            };

            if (this.method === 'POST' || this.method === 'PUT' || this.method === 'PATCH') {
              options.body = JSON.stringify(this.body ? JSON.parse(this.body) : {});
            }

            const response = await fetch(this.url, options);
            clearTimeout(this.timeoutId); // Clear timeout if response is received

            if (!response.ok) {
              throw new Error(`HTTP error! Status: ${response.status}`);
            }

            const responseText = await response.text(); // Read response as text
            console.log('Raw response:', responseText); // Log the raw response

            this.loading = false; // Hide loading indicator
            loader.style.display = "none"; // Hide spinner

            if (response.headers.get('Content-Type').includes('application/json')) {
              // Clean up the response to make it valid JSON
              const cleanedResponseText = responseText.replace(/ObjectId\("([^"]+)"\)/, '\$1');
              const responseData = JSON.parse(cleanedResponseText); // Parse JSON
              this.response = JSON.stringify(responseData, null, 2);
            } else {
              this.response = responseText; // Handle as plain text
            }
          } catch (error) {
            this.loading = false;
            loader.style.display = "none"; // Hide spinner if error occurs
            this.response = `Error: ${error.message}`;
          }
        },
        parseHeaders(headersString) {
          try {
            return JSON.parse(headersString);
          } catch (e) {
            return {};
          }
        },

        copyResponse() {
          navigator.clipboard.writeText(this.response).then(() => {
            this.copySuccess = true;
            setTimeout(() => {
              this.copySuccess = false;
            }, 5000);
          }).catch(err => {
            console.error('Failed to copy: ', err);
          });
        }
      }
    }
  </script>
</body>

</html>
"#;
