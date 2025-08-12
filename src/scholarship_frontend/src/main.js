console.log('🎓 Scholarship Matcher Loading...');

document.addEventListener('DOMContentLoaded', function() {
  console.log('DOM ready, creating app...');
  
  const root = document.getElementById('root');
  if (!root) {
    console.error('Root element not found!');
    return;
  }

  // Create UI
  root.innerHTML = `
    <div style="max-width: 1200px; margin: 0 auto; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;">
      
      <header style="background: white; padding: 30px; border-radius: 15px; margin-bottom: 30px; box-shadow: 0 20px 40px rgba(0,0,0,0.1); text-align: center;">
        <h1 style="color: #333; margin: 0 0 10px 0; font-size: 2.5em;">🎓 Scholarship Matcher</h1>
        <p style="color: #666; margin: 0 0 20px 0; font-size: 1.2em;">Find your perfect scholarship match using AI on Internet Computer</p>
        
        <div style="margin: 20px 0;">
          <span id="status" style="background: #28a745; color: white; padding: 12px 24px; border-radius: 25px; font-size: 16px;">
            ✅ Frontend Active
          </span>
        </div>
        
        <div>
          <button onclick="testConnection()" style="margin: 5px; padding: 12px 20px; background: #6c757d; color: white; border: none; border-radius: 8px; cursor: pointer;">
            🔌 Test Backend
          </button>
          <button onclick="loadScholarships()" style="margin: 5px; padding: 12px 20px; background: #28a745; color: white; border: none; border-radius: 8px; cursor: pointer;">
            📚 Load Scholarships
          </button>
          <button onclick="showRegister()" style="margin: 5px; padding: 12px 20px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer;">
            👤 Register
          </button>
        </div>
      </header>

      <div style="background: white; padding: 30px; border-radius: 15px; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
        
        <div id="register-form" style="display: none; margin-bottom: 30px; padding: 20px; background: #f8f9fa; border-radius: 10px;">
          <h3>👤 User Registration</h3>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <input type="text" id="name" placeholder="Name" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="email" id="email" placeholder="Email" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="text" id="university" placeholder="University" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="text" id="major" placeholder="Major" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          <button onclick="registerUser()" style="padding: 15px 30px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer;">
            🚀 Register
          </button>
        </div>

        <div>
          <h3 style="text-align: center;">🎓 Available Scholarships</h3>
          <div id="scholarships">
            <div style="text-align: center; padding: 60px; color: #666; background: #f8f9fa; border-radius: 10px;">
              <h4>🌟 Welcome to Scholarship Matcher!</h4>
              <p>Click "Load Scholarships" to browse opportunities</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  `;

  console.log('✅ UI created successfully!');
});

// Global functions
window.testConnection = async function() {
  try {
    console.log('Testing connection...');
    updateStatus('🔄 Testing...', '#ffc107');
    
    // Try to import IC backend
    const { scholarship_backend } = await import('../../../.dfx/local/canisters/scholarship_backend/index.js');
    const response = await scholarship_backend.test_connection();
    
    alert('✅ Backend connected! Response: ' + JSON.stringify(response));
    updateStatus('🟢 Connected', '#28a745');
  } catch (error) {
    console.error('Backend error:', error);
    updateStatus('🔴 Backend Failed', '#dc3545');
    alert('❌ Error: ' + error.message);
  }
}

window.loadScholarships = async function() {
  try {
    console.log('Loading scholarships...');
    updateStatus('🔄 Loading...', '#ffc107');
    
    // Try to load from IC backend
    const { scholarship_backend } = await import('../../../.dfx/local/canisters/scholarship_backend/index.js');
    const scholarships = await scholarship_backend.get_all_scholarships();
    
    console.log('Loaded scholarships:', scholarships);
    displayScholarships(scholarships);
    updateStatus('🟢 Loaded from IC', '#28a745');
    alert(`✅ ${scholarships.length} scholarships loaded from IC backend!`);
  } catch (error) {
    console.log('Backend failed, using sample data:', error);
    displaySample();
    updateStatus('🟡 Sample Data', '#ffc107');
    alert('⚠️ Using sample data. Backend connection failed: ' + error.message);
  }
}

window.showRegister = function() {
  const form = document.getElementById('register-form');
  form.style.display = form.style.display === 'none' ? 'block' : 'none';
}

window.registerUser = function() {
  const name = document.getElementById('name').value;
  const email = document.getElementById('email').value;
  
  if (!name || !email) {
    alert('Please fill name and email');
    return;
  }
  
  alert('✅ Registration successful for: ' + name);
  document.getElementById('register-form').style.display = 'none';
  displaySample();
}

function updateStatus(text, color) {
  const status = document.getElementById('status');
  status.textContent = text;
  status.style.background = color;
}

function displayScholarships(response) {
  if (!response || !response.includes('scholarships')) {
    displaySample();
    return;
  }
  
  const lines = response.split('\\n').filter(line => line.includes(' - '));
  let html = '';
  
  lines.forEach(function(line, index) {
    const parts = line.split(' - ');
    const name = parts[0].replace('- ', '').trim();
    const provider = parts[1] ? parts[1].replace('Provider: ', '') : 'Unknown';
    const country = parts[2] ? parts[2].replace('Country: ', '') : 'Unknown';
    const score = Math.floor(Math.random() * 30) + 70;
    
    html += `
      <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; margin-bottom: 15px; background: #fff;">
        <h4 style="color: #007bff; margin: 0 0 10px 0;">${name}</h4>
        <p style="margin: 5px 0; color: #666;">🏛️ ${provider}</p>
        <p style="margin: 5px 0; color: #666;">🌍 ${country}</p>
        <span style="background: #28a745; color: white; padding: 6px 12px; border-radius: 15px; font-size: 14px;">
          📊 ${score}% Match
        </span>
      </div>
    `;
  });
  
  document.getElementById('scholarships').innerHTML = html;
}

function displayScholarships(scholarships) {
  let html = '';
  scholarships.forEach(scholarship => {
    html += `
      <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; margin-bottom: 15px; background: #fff;">
        <h4 style="color: #007bff; margin: 0 0 10px 0;">${scholarship.name}</h4>
        <p style="margin: 5px 0; color: #666;">🏛️ ${scholarship.provider}</p>
        <p style="margin: 5px 0; color: #666;">🌍 ${scholarship.country}</p>
        <p style="margin: 5px 0; color: #666;">💰 ${scholarship.funding_type}</p>
        <p style="margin: 5px 0; color: #666;">📚 ${scholarship.field_of_study}</p>
        <p style="margin: 10px 0; color: #333;">${scholarship.description}</p>
        <span style="background: #007bff; color: white; padding: 6px 12px; border-radius: 15px; font-size: 14px;">
          🎓 ${scholarship.degree_level}
        </span>
      </div>
    `;
  });
  
  document.getElementById('scholarships').innerHTML = html;
}

function displaySample() {
  document.getElementById('scholarships').innerHTML = `
    <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; margin-bottom: 15px; background: #fff;">
      <h4 style="color: #007bff; margin: 0 0 10px 0;">🇮🇩 LPDP Scholarship 2025</h4>
      <p style="margin: 5px 0; color: #666;">🏛️ Indonesian Government</p>
      <p style="margin: 5px 0; color: #666;">🌍 Indonesia</p>
      <span style="background: #28a745; color: white; padding: 6px 12px; border-radius: 15px; font-size: 14px;">
        📊 95% Match
      </span>
    </div>
    <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; margin-bottom: 15px; background: #fff;">
      <h4 style="color: #007bff; margin: 0 0 10px 0;">🇬🇧 Chevening Scholarship 2025</h4>
      <p style="margin: 5px 0; color: #666;">🏛️ UK Government</p>
      <p style="margin: 5px 0; color: #666;">🌍 United Kingdom</p>
      <span style="background: #17a2b8; color: white; padding: 6px 12px; border-radius: 15px; font-size: 14px;">
        📊 78% Match
      </span>
    </div>
    <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; margin-bottom: 15px; background: #fff;">
      <h4 style="color: #007bff; margin: 0 0 10px 0;">🇺🇸 Fulbright Scholarship 2025</h4>
      <p style="margin: 5px 0; color: #666;">🏛️ US Department of State</p>
      <p style="margin: 5px 0; color: #666;">🌍 United States</p>
      <span style="background: #ffc107; color: #212529; padding: 6px 12px; border-radius: 15px; font-size: 14px;">
        📊 82% Match
      </span>
    </div>
  `;
}
