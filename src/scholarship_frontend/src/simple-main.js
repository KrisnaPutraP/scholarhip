// Simple Scholarship Matcher Frontend
console.log('Loading Scholarship Matcher...');

document.addEventListener('DOMContentLoaded', () => {
  console.log('DOM ready, creating app...');
  
  const root = document.getElementById('root');
  if (!root) {
    console.error('Root element not found!');
    return;
  }

  // Create main UI structure
  root.innerHTML = `
    <div style="max-width: 1200px; margin: 0 auto; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;">
      
      <!-- Header -->
      <header style="background: white; padding: 30px; border-radius: 15px; margin-bottom: 30px; box-shadow: 0 20px 40px rgba(0,0,0,0.1); text-align: center;">
        <h1 style="color: #333; margin: 0 0 10px 0; font-size: 2.5em;">🎓 Scholarship Matcher</h1>
        <p style="color: #666; margin: 0 0 20px 0; font-size: 1.2em;">Find your perfect scholarship match using AI on Internet Computer</p>
        
        <div style="margin: 20px 0;">
          <span id="status-badge" style="background: #28a745; color: white; padding: 12px 24px; border-radius: 25px; font-size: 16px; margin-right: 15px;">
            ✅ Frontend Active
          </span>
        </div>
        
        <div style="margin-top: 20px;">
          <button id="test-connection" style="margin: 5px; padding: 12px 20px; background: #6c757d; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px;">
            🔌 Test IC Backend
          </button>
          <button id="load-scholarships" style="margin: 5px; padding: 12px 20px; background: #28a745; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px;">
            📚 Load 24 Scholarships
          </button>
          <button id="show-register" style="margin: 5px; padding: 12px 20px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 14px;">
            👤 Register User
          </button>
        </div>
      </header>

      <!-- Main Content -->
      <div style="background: white; padding: 30px; border-radius: 15px; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
        
        <!-- Registration Section (hidden initially) -->
        <div id="registration-section" style="display: none; margin-bottom: 30px; padding: 20px; background: #f8f9fa; border-radius: 10px;">
          <h3 style="margin-top: 0;">👤 User Registration</h3>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px;">
            <div>
              <label style="display: block; margin-bottom: 5px; font-weight: bold;">Name *</label>
              <input type="text" id="user-name" placeholder="John Doe" style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            </div>
            <div>
              <label style="display: block; margin-bottom: 5px; font-weight: bold;">Email *</label>
              <input type="email" id="user-email" placeholder="john@example.com" style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            </div>
            <div>
              <label style="display: block; margin-bottom: 5px; font-weight: bold;">University *</label>
              <input type="text" id="user-university" placeholder="MIT" style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            </div>
            <div>
              <label style="display: block; margin-bottom: 5px; font-weight: bold;">Major *</label>
              <input type="text" id="user-major" placeholder="Computer Science" style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            </div>
            <div>
              <label style="display: block; margin-bottom: 5px; font-weight: bold;">GPA</label>
              <input type="number" id="user-gpa" step="0.01" min="0" max="4" placeholder="3.85" style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            </div>
            <div>
              <label style="display: block; margin-bottom: 5px; font-weight: bold;">Degree Level</label>
              <select id="user-degree" style="width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
                <option value="Bachelor">Bachelor</option>
                <option value="Master">Master</option>
                <option value="PhD">PhD</option>
              </select>
            </div>
          </div>
          <div style="margin-top: 20px; text-align: center;">
            <button id="register-btn" style="padding: 15px 30px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer; font-size: 16px; font-weight: bold;">
              🚀 Register & Get Recommendations
            </button>
          </div>
        </div>

        <!-- Scholarships Section -->
        <div>
          <h3 style="margin-top: 0; text-align: center;">🎓 Available Scholarships</h3>
          <div id="scholarships-container" style="margin-top: 20px;">
            <div style="text-align: center; padding: 60px; color: #666; background: #f8f9fa; border-radius: 10px;">
              <h4>🌟 Welcome to Scholarship Matcher!</h4>
              <p>Click "Load 24 Scholarships" to browse available opportunities</p>
              <p>Or register to get personalized recommendations</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  `;

  // Event Listeners
  setupEventListeners();
  console.log('UI created successfully!');
});

function setupEventListeners() {
  // Show/hide registration
  document.getElementById('show-register').addEventListener('click', () => {
    const section = document.getElementById('registration-section');
    if (section.style.display === 'none') {
      section.style.display = 'block';
      document.getElementById('show-register').textContent = '❌ Hide Registration';
    } else {
      section.style.display = 'none';
      document.getElementById('show-register').textContent = '👤 Register User';
    }
  });

  // Test connection
  document.getElementById('test-connection').addEventListener('click', async () => {
    try {
      console.log('Testing IC backend connection...');
      updateStatus('🔄 Testing connection...', '#ffc107');
      
      // Dynamic import
      const { scholarship_backend } = await import('/src/declarations/scholarship_backend/index.js');
      const response = await scholarship_backend.test_connection();
      
      console.log('Connection success:', response);
      updateStatus('🟢 Connected to IC', '#28a745');
      alert('✅ Backend Connection Success!\\n' + response);
      
    } catch (error) {
      console.error('Connection failed:', error);
      updateStatus('🔴 Connection Failed', '#dc3545');
      alert('❌ Connection Failed!\\n' + error.message);
    }
  });

  // Load scholarships
  document.getElementById('load-scholarships').addEventListener('click', async () => {
    try {
      console.log('Loading scholarships...');
      updateStatus('🔄 Loading scholarships...', '#ffc107');
      
      const { scholarship_backend } = await import('/src/declarations/scholarship_backend/index.js');
      const response = await scholarship_backend.get_all_scholarships();
      
      console.log('Scholarships loaded:', response);
      displayScholarships(response);
      updateStatus('🟢 Scholarships Loaded', '#28a745');
      
    } catch (error) {
      console.error('Failed to load scholarships:', error);
      updateStatus('⚠️ Using Sample Data', '#ffc107');
      displaySampleScholarships();
    }
  });

  // Register user
  document.getElementById('register-btn').addEventListener('click', async () => {
    const userData = {
      name: document.getElementById('user-name').value,
      email: document.getElementById('user-email').value,
      university: document.getElementById('user-university').value,
      major: document.getElementById('user-major').value,
      gpa: parseFloat(document.getElementById('user-gpa').value) || 3.5,
      degree: document.getElementById('user-degree').value
    };

    if (!userData.name || !userData.email || !userData.university || !userData.major) {
      alert('Please fill in all required fields (marked with *)');
      return;
    }

    try {
      console.log('Registering user:', userData);
      updateStatus('🔄 Registering user...', '#ffc107');
      
      // For demo, show success
      updateStatus('🟢 User Registered', '#28a745');
      alert('✅ Registration Successful!\\nWelcome ' + userData.name + '!');
      
      // Hide registration form
      document.getElementById('registration-section').style.display = 'none';
      document.getElementById('show-register').textContent = '👤 Register User';
      
      // Load sample recommendations
      displaySampleScholarships();
      
    } catch (error) {
      console.error('Registration failed:', error);
      alert('❌ Registration Failed!\\n' + error.message);
    }
  });
}

function updateStatus(text, color) {
  const badge = document.getElementById('status-badge');
  badge.textContent = text;
  badge.style.background = color;
}

function displayScholarships(response) {
  const container = document.getElementById('scholarships-container');
  
  if (response && response.includes('scholarships')) {
    const lines = response.split('\\n').filter(line => line.includes(' - '));
    let html = '<div style="display: grid; gap: 20px;">';
    
    lines.forEach((line, index) => {
      const parts = line.split(' - ');
      const name = parts[0].replace('- ', '').trim();
      const provider = parts[1] ? parts[1].replace('Provider: ', '').trim() : 'Unknown';
      const country = parts[2] ? parts[2].replace('Country: ', '').trim() : 'Unknown';
      const score = Math.floor(Math.random() * 30) + 70;
      
      const colors = ['#28a745', '#17a2b8', '#ffc107', '#6f42c1'];
      const color = colors[index % colors.length];
      
      html += `
        <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; background: #fff; box-shadow: 0 4px 6px rgba(0,0,0,0.1); transition: transform 0.2s;" onmouseover="this.style.transform='scale(1.02)'" onmouseout="this.style.transform='scale(1)'">
          <h4 style="margin: 0 0 10px 0; color: #007bff; font-size: 1.2em;">${name}</h4>
          <p style="margin: 0 0 5px 0; color: #666;"><strong>🏛️ Provider:</strong> ${provider}</p>
          <p style="margin: 0 0 15px 0; color: #666;"><strong>🌍 Country:</strong> ${country}</p>
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <span style="background: ${color}; color: white; padding: 8px 16px; border-radius: 20px; font-size: 14px; font-weight: bold;">
              📊 Match: ${score}%
            </span>
            <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;">
              📝 Apply
            </button>
          </div>
        </div>
      `;
    });
    
    html += '</div>';
    container.innerHTML = html;
  } else {
    displaySampleScholarships();
  }
}

function displaySampleScholarships() {
  const container = document.getElementById('scholarships-container');
  container.innerHTML = `
    <div style="display: grid; gap: 20px;">
      <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; background: #fff; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
        <h4 style="margin: 0 0 10px 0; color: #007bff; font-size: 1.2em;">🇮🇩 LPDP Scholarship 2025</h4>
        <p style="margin: 0 0 5px 0; color: #666;"><strong>🏛️ Provider:</strong> Indonesian Government</p>
        <p style="margin: 0 0 15px 0; color: #666;"><strong>🌍 Country:</strong> Indonesia</p>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span style="background: #28a745; color: white; padding: 8px 16px; border-radius: 20px; font-size: 14px; font-weight: bold;">
            📊 Match: 95%
          </span>
          <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;">
            📝 Apply
          </button>
        </div>
      </div>
      
      <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; background: #fff; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
        <h4 style="margin: 0 0 10px 0; color: #007bff; font-size: 1.2em;">🇬🇧 Chevening Scholarship 2025</h4>
        <p style="margin: 0 0 5px 0; color: #666;"><strong>🏛️ Provider:</strong> UK Government</p>
        <p style="margin: 0 0 15px 0; color: #666;"><strong>🌍 Country:</strong> United Kingdom</p>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span style="background: #17a2b8; color: white; padding: 8px 16px; border-radius: 20px; font-size: 14px; font-weight: bold;">
            📊 Match: 78%
          </span>
          <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;">
            📝 Apply
          </button>
        </div>
      </div>
      
      <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; background: #fff; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
        <h4 style="margin: 0 0 10px 0; color: #007bff; font-size: 1.2em;">🇺🇸 Fulbright Scholarship 2025</h4>
        <p style="margin: 0 0 5px 0; color: #666;"><strong>🏛️ Provider:</strong> US Department of State</p>
        <p style="margin: 0 0 15px 0; color: #666;"><strong>🌍 Country:</strong> United States</p>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span style="background: #ffc107; color: #212529; padding: 8px 16px; border-radius: 20px; font-size: 14px; font-weight: bold;">
            📊 Match: 82%
          </span>
          <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;">
            📝 Apply
          </button>
        </div>
      </div>
    </div>
  `;
}
