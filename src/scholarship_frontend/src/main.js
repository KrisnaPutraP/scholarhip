console.log('🎓 Scholarship Matcher Loading...');

// Import backend
import { scholarship_backend } from '../../declarations/scholarship_backend/index.js';

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
          <button id="testBtn" style="margin: 5px; padding: 12px 20px; background: #6c757d; color: white; border: none; border-radius: 8px; cursor: pointer;">
            🔌 Test Backend
          </button>


          <button id="matchBtn" style="margin: 5px; padding: 12px 20px; background: #17a2b8; color: white; border: none; border-radius: 8px; cursor: pointer;">
            🎯 Get My Matches
          </button>
          <button id="profileBtn" style="margin: 5px; padding: 12px 20px; background: #6f42c1; color: white; border: none; border-radius: 8px; cursor: pointer;">
            👤 Check Profile
          </button>
          <button id="registerBtn" style="margin: 5px; padding: 12px 20px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer;">
            � Register
          </button>
        </div>
      </header>

      <div style="background: white; padding: 30px; border-radius: 15px; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
        
        <div id="register-form" style="display: none; margin-bottom: 30px; padding: 20px; background: #f8f9fa; border-radius: 10px;">
          <h3>👤 User Registration</h3>
          
          <!-- Basic Info -->
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <input type="text" id="name" placeholder="Full Name" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="email" id="email" placeholder="Email" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          
          <!-- Education Info -->
          <h4 style="margin: 20px 0 10px 0;">📚 Education</h4>
          <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <select id="degree-level" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
              <option value="">Select Degree Level</option>
              <option value="HighSchool">High School</option>
              <option value="Bachelor">Bachelor</option>
              <option value="Master">Master</option>
              <option value="PhD">PhD</option>
            </select>
            <input type="text" id="major" placeholder="Major/Field" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="number" id="gpa" placeholder="GPA (0-4.0)" step="0.01" min="0" max="4" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          
          <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <input type="text" id="university" placeholder="University" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="number" id="graduation-year" placeholder="Graduation Year" min="2020" max="2030" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="text" id="country" placeholder="Country" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          
          <!-- Skills -->
          <h4 style="margin: 20px 0 10px 0;">💪 Skills</h4>
          <input type="text" id="skills" placeholder="Skills (comma separated, e.g., Python, Research, English)" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px; width: 100%; margin-bottom: 20px;">
          
          <!-- Preferences -->
          <h4 style="margin: 20px 0 10px 0;">🎯 Preferences</h4>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <input type="text" id="preferred-countries" placeholder="Preferred Countries (comma separated)" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="text" id="preferred-fields" placeholder="Preferred Fields (comma separated)" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <select id="scholarship-type" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
              <option value="">Select Scholarship Type</option>
              <option value="FullScholarship">Full Scholarship</option>
              <option value="PartialScholarship">Partial Scholarship</option>
              <option value="ResearchGrant">Research Grant</option>
              <option value="ExchangeProgram">Exchange Program</option>
            </select>
            <input type="number" id="min-amount" placeholder="Min Amount ($)" min="0" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          
          <button id="submitRegBtn" style="padding: 15px 30px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer;">
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
  
  // Add event listeners
  document.getElementById('testBtn').addEventListener('click', testConnection);


  document.getElementById('matchBtn').addEventListener('click', loadScholarshipsWithMatching);
  document.getElementById('profileBtn').addEventListener('click', checkProfile);
  document.getElementById('registerBtn').addEventListener('click', showRegister);
  document.getElementById('submitRegBtn').addEventListener('click', registerUser);
  
  console.log('✅ Event listeners attached!');
  
  // Auto-initialize and load scholarships on startup
  setTimeout(() => {
    autoInitializeAndLoad();
  }, 1000);
});

// Auto-initialization function
async function autoInitializeAndLoad() {
  try {
    console.log('🚀 Auto-initializing scholarship system...');
    updateStatus('🔄 Starting System...', '#ffc107');
    
    // Step 1: Test backend connection
    console.log('Step 1: Testing backend connection...');
    const connectionTest = await scholarship_backend.test_connection();
    console.log('Connection test result:', connectionTest);
    
    if (!connectionTest || connectionTest.includes('Error')) {
      throw new Error('Backend connection failed');
    }
    
    // Step 2: Check if scholarships exist
    console.log('Step 2: Checking existing scholarships...');
    const existingScholarships = await scholarship_backend.get_all_scholarships();
    console.log('Existing scholarships check:', existingScholarships);
    
    // Step 3: Initialize if no scholarships found
    if (!existingScholarships || existingScholarships.includes('No scholarships found') || existingScholarships.trim() === '') {
      console.log('Step 3: No scholarships found, initializing sample data...');
      updateStatus('🔄 Initializing Data...', '#ffc107');
      
      const initResult = await scholarship_backend.manual_init_scholarships();
      console.log('Initialization result:', initResult);
      
      if (!initResult || initResult.includes('Error')) {
        throw new Error('Failed to initialize sample data');
      }
    }
    
    // Step 4: Load scholarships
    console.log('Step 4: Loading scholarships...');
    updateStatus('🔄 Loading Scholarships...', '#28a745');
    
    const scholarshipsResponse = await scholarship_backend.get_all_scholarships();
    const scholarships = parseScholarshipResponse(scholarshipsResponse);
    
    if (scholarships && scholarships.length > 0) {
      displayScholarships(scholarships);
      updateStatus(`🟢 Ready (${scholarships.length} scholarships)`, '#28a745');
      console.log(`✅ Auto-initialization complete! Loaded ${scholarships.length} scholarships.`);
      
      // Show success message briefly
      showAutoLoadSuccess(scholarships.length);
    } else {
      throw new Error('No scholarships could be loaded');
    }
    
  } catch (error) {
    console.error('❌ Auto-initialization failed:', error);
    updateStatus('🔴 System Error', '#dc3545');
    
    // Show fallback UI with error message and manual controls
    showErrorFallback(error.message);
  }
}

function showAutoLoadSuccess(count) {
  // Create a temporary success banner
  const banner = document.createElement('div');
  banner.style.cssText = `
    position: fixed;
    top: 20px;
    right: 20px;
    background: linear-gradient(135deg, #28a745, #20c997);
    color: white;
    padding: 15px 20px;
    border-radius: 10px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
    z-index: 1000;
    font-weight: bold;
    animation: slideIn 0.5s ease-out;
  `;
  banner.innerHTML = `
    <div style="display: flex; align-items: center;">
      <span style="font-size: 20px; margin-right: 10px;">✅</span>
      <div>
        <div>System Ready!</div>
        <div style="font-size: 12px; opacity: 0.9;">${count} scholarships loaded automatically</div>
      </div>
    </div>
  `;
  
  // Add animation keyframes
  if (!document.getElementById('banner-styles')) {
    const style = document.createElement('style');
    style.id = 'banner-styles';
    style.textContent = `
      @keyframes slideIn {
        from { transform: translateX(100%); opacity: 0; }
        to { transform: translateX(0); opacity: 1; }
      }
      @keyframes slideOut {
        from { transform: translateX(0); opacity: 1; }
        to { transform: translateX(100%); opacity: 0; }
      }
    `;
    document.head.appendChild(style);
  }
  
  document.body.appendChild(banner);
  
  // Remove banner after 4 seconds
  setTimeout(() => {
    banner.style.animation = 'slideOut 0.5s ease-in';
    setTimeout(() => {
      if (banner.parentNode) {
        banner.parentNode.removeChild(banner);
      }
    }, 500);
  }, 4000);
}

function showErrorFallback(errorMessage) {
  document.getElementById('scholarships').innerHTML = `
    <div style="text-align: center; padding: 40px; background: #f8d7da; border: 2px solid #dc3545; border-radius: 15px; color: #721c24;">
      <h3 style="color: #dc3545; margin: 0 0 20px 0;">⚠️ System Initialization Failed</h3>
      
      <div style="background: white; padding: 20px; border-radius: 10px; margin: 20px 0; text-align: left;">
        <h4 style="color: #dc3545; margin: 0 0 10px 0;">Error Details:</h4>
        <p style="margin: 0; font-family: monospace; color: #721c24; word-break: break-word;">${errorMessage}</p>
      </div>
      
      <div style="margin: 20px 0;">
        <h4 style="color: #dc3545;">Possible Solutions:</h4>
        <div style="text-align: left; max-width: 500px; margin: 0 auto;">
          <p>• <strong>Backend not running:</strong> Make sure dfx is running (<code>dfx start</code>)</p>
          <p>• <strong>Canister not deployed:</strong> Deploy the backend (<code>dfx deploy scholarship_backend</code>)</p>
          <p>• <strong>Network issues:</strong> Check your internet connection</p>
          <p>• <strong>Browser cache:</strong> Try refreshing the page (Ctrl+F5)</p>
        </div>
      </div>
      
      <div style="margin-top: 30px;">
        <h4 style="color: #dc3545;">Manual Recovery:</h4>
        <div style="display: flex; gap: 10px; justify-content: center; flex-wrap: wrap; margin-top: 15px;">
          <button onclick="testConnection()" style="padding: 10px 15px; background: #6c757d; color: white; border: none; border-radius: 6px; cursor: pointer;">
            🔌 Test Connection
          </button>
          <button onclick="initializeData()" style="padding: 10px 15px; background: #ffc107; color: #212529; border: none; border-radius: 6px; cursor: pointer;">
            🔄 Initialize Data
          </button>
          <button onclick="loadScholarships()" style="padding: 10px 15px; background: #28a745; color: white; border: none; border-radius: 6px; cursor: pointer;">
            📚 Load Scholarships
          </button>
          <button onclick="location.reload()" style="padding: 10px 15px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;">
            🔄 Refresh Page
          </button>
        </div>
      </div>
    </div>
  `;
}

// Functions
async function testConnection() {
  try {
    console.log('Testing connection...');
    updateStatus('🔄 Testing...', '#ffc107');
    
    // Use imported backend
    const response = await scholarship_backend.test_connection();
    
    alert('✅ Backend connected! Response: ' + JSON.stringify(response));
    updateStatus('🟢 Connected', '#28a745');
  } catch (error) {
    console.error('Backend error:', error);
    updateStatus('🔴 Backend Failed', '#dc3545');
    alert('❌ Error: ' + error.message);
  }
}

async function initializeData() {
  try {
    console.log('Initializing sample data...');
    updateStatus('🔄 Initializing...', '#ffc107');
    
    // Call manual init function
    const response = await scholarship_backend.manual_init_scholarships();
    
    console.log('Initialization response:', response);
    updateStatus('🟢 Data Initialized', '#28a745');
    alert('✅ ' + response);
  } catch (error) {
    console.error('Initialization error:', error);
    updateStatus('🔴 Init Failed', '#dc3545');
    alert('❌ Initialization Error: ' + error.message);
  }
}

async function checkProfile() {
  try {
    console.log('Checking user profile...');
    updateStatus('🔄 Checking Profile...', '#ffc107');
    
    const profile = await scholarship_backend.get_my_profile();
    console.log('Profile response:', profile);
    
    if (profile.includes('Error:') || profile.includes('not found')) {
      alert('❌ No user profile found!\n\nResponse: ' + profile + '\n\nPlease register first.');
      updateStatus('🔴 Not Registered', '#dc3545');
    } else {
      alert('✅ User profile found!\n\nProfile: ' + profile);
      updateStatus('🟢 Profile Found', '#28a745');
    }
  } catch (error) {
    console.error('Profile check error:', error);
    alert('❌ Error checking profile: ' + error.message);
    updateStatus('🔴 Profile Check Failed', '#dc3545');
  }
}

async function loadScholarshipsWithMatching() {
  try {
    console.log('Loading scholarships with matching...');
    updateStatus('🔄 Loading Matches...', '#ffc107');
    
    // First, try get_my_matches (simpler function)
    console.log('Calling get_my_matches...');
    let recommendations = await scholarship_backend.get_my_matches();
    console.log('get_my_matches response:', recommendations);
    
    // If that doesn't work, try get_my_recommendations
    if (!recommendations || recommendations.includes('Error:') || recommendations.includes('not found')) {
      console.log('Trying get_my_recommendations with limit...');
      recommendations = await scholarship_backend.get_my_recommendations([10]);
      console.log('get_my_recommendations response:', recommendations);
    }
    
    console.log('Final recommendations:', recommendations);
    console.log('Response type:', typeof recommendations);
    
    // If we have valid recommendations, parse and display them
    if (recommendations && !recommendations.includes('Error:') && !recommendations.includes('not found')) {
      console.log('Processing valid recommendations...');
      parseAndDisplayRecommendations(recommendations);
      updateStatus('🟢 Matches Loaded', '#28a745');
    } else {
      // Show the error message for debugging
      console.log('No valid recommendations, error:', recommendations);
      alert('⚠️ Backend response: ' + recommendations + '\n\nPlease make sure you are registered first.');
      
      // Fallback to regular scholarships
      console.log('Loading regular scholarships as fallback...');
      loadScholarships();
    }
  } catch (error) {
    console.error('Error loading matches:', error);
    alert('❌ Error getting matches: ' + error.message + '\n\nFalling back to regular scholarships.');
    // Fallback to regular scholarships
    loadScholarships();
  }
}

function parseAndDisplayRecommendations(response) {
  console.log('Parsing recommendations:', response);
  
  // Check if it's an error message
  if (response.includes('Error:') || response.includes('not found')) {
    alert('⚠️ ' + response + '\nPlease register first to get personalized matches.');
    loadScholarships();
    return;
  }
  
  // Parse the recommendation response - look for pattern like "ScholarshipName: XX.X% match"
  const text = response.replace(/🎯/g, '').trim();
  
  // Split by commas and extract scholarship data
  const scholarshipMatches = [];
  const parts = text.split(',');
  
  parts.forEach(part => {
    const trimmed = part.trim();
    // Look for pattern: "Scholarship Name: XX.X% match" or "Scholarship Name 2025: XX.X% match"
    const match = trimmed.match(/(.+?):\s*([\d.]+)%\s*match/i);
    if (match) {
      const name = match[1].trim();
      const percentage = parseFloat(match[2]);
      
      if (name && !isNaN(percentage)) {
        scholarshipMatches.push({
          name: name,
          percentage: percentage
        });
      }
    }
  });
  
  console.log('Parsed scholarships:', scholarshipMatches);
  
  if (scholarshipMatches.length === 0) {
    alert('📊 Could not parse match scores. Raw response: ' + response);
    loadScholarships();
    return;
  }
  
  // Sort by percentage (highest first)
  scholarshipMatches.sort((a, b) => b.percentage - a.percentage);
  
  let html = `
    <div style="text-align: center; margin-bottom: 30px;">
      <h3 style="color: #007bff; margin: 0 0 10px 0;">🎯 Your Personalized Matches</h3>
      <p style="color: #666; margin: 0 0 20px 0;">AI-powered recommendations based on your profile, education, and preferences</p>
      <div style="background: #f8f9fa; padding: 15px; border-radius: 10px; margin-bottom: 20px;">
        <strong>Found ${scholarshipMatches.length} matching scholarships</strong> • 
        Sorted by compatibility score
      </div>
    </div>
  `;
  
  scholarshipMatches.forEach((scholarship, index) => {
    const percentage = scholarship.percentage;
    const name = scholarship.name;
    
    // Determine colors and styling based on match percentage
    let bgColor, badgeColor, emoji, rankText;
    if (percentage >= 75) {
      bgColor = '#d4edda';
      badgeColor = '#28a745';
      emoji = '🏆';
      rankText = 'Excellent Match';
    } else if (percentage >= 65) {
      bgColor = '#fff3cd';
      badgeColor = '#ffc107';
      emoji = '🥈';
      rankText = 'Very Good Match';
    } else if (percentage >= 55) {
      bgColor = '#ffeaa7';
      badgeColor = '#fd7e14';
      emoji = '🥉';
      rankText = 'Good Match';
    } else {
      bgColor = '#f8d7da';
      badgeColor = '#6c757d';
      emoji = '📋';
      rankText = 'Potential Match';
    }
    
    html += `
      <div style="
        border: 2px solid ${badgeColor}; 
        border-radius: 15px; 
        padding: 25px; 
        margin-bottom: 20px; 
        background: ${bgColor};
        box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        transition: transform 0.2s;
      " onmouseover="this.style.transform='translateY(-2px)'" onmouseout="this.style.transform='translateY(0)'">
        
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
          <div style="display: flex; align-items: center;">
            <span style="font-size: 24px; margin-right: 10px;">${emoji}</span>
            <div>
              <h4 style="color: #333; margin: 0; font-size: 18px;">${name}</h4>
              <p style="color: #666; margin: 5px 0 0 0; font-size: 14px;">${rankText}</p>
            </div>
          </div>
          
          <div style="text-align: right;">
            <div style="
              background: ${badgeColor}; 
              color: white; 
              padding: 10px 15px; 
              border-radius: 25px; 
              font-size: 18px; 
              font-weight: bold;
              min-width: 100px;
            ">
              ${percentage.toFixed(1)}%
            </div>
            <p style="color: #666; margin: 5px 0 0 0; font-size: 12px;">Rank #${index + 1}</p>
          </div>
        </div>
        
        <div style="background: rgba(255,255,255,0.7); padding: 10px; border-radius: 8px; margin-top: 10px;">
          <p style="margin: 0; color: #555; font-size: 14px;">
            <strong>💡 Match Analysis:</strong> This scholarship aligns with your profile based on education level, field of study, and preferences.
          </p>
        </div>
      </div>
    `;
  });
  
  // Add summary statistics
  const avgMatch = (scholarshipMatches.reduce((sum, s) => sum + s.percentage, 0) / scholarshipMatches.length).toFixed(1);
  const topMatch = scholarshipMatches[0].percentage.toFixed(1);
  
  html += `
    <div style="
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
      padding: 20px;
      border-radius: 15px;
      text-align: center;
      margin-top: 20px;
    ">
      <h4 style="margin: 0 0 10px 0;">📊 Match Summary</h4>
      <div style="display: flex; justify-content: space-around; flex-wrap: wrap;">
        <div style="margin: 5px;">
          <strong>Best Match:</strong> ${topMatch}%
        </div>
        <div style="margin: 5px;">
          <strong>Average Match:</strong> ${avgMatch}%
        </div>
        <div style="margin: 5px;">
          <strong>Total Found:</strong> ${scholarshipMatches.length} scholarships
        </div>
      </div>
    </div>
  `;
  
  document.getElementById('scholarships').innerHTML = html;
  updateStatus('🎯 Matches Found', '#28a745');
}

async function loadScholarships() {
  try {
    console.log('Loading scholarships...');
    updateStatus('🔄 Loading...', '#ffc107');
    
    // Use imported backend
    const response = await scholarship_backend.get_all_scholarships();
    
    // Parse the string response into array format
    const scholarships = parseScholarshipResponse(response);
    
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

function showRegister() {
  const form = document.getElementById('register-form');
  form.style.display = form.style.display === 'none' ? 'block' : 'none';
}

async function registerUser() {
  try {
    // Collect basic info
    const name = document.getElementById('name').value;
    const email = document.getElementById('email').value;
    
    // Collect education info
    const degreeLevel = document.getElementById('degree-level').value;
    const major = document.getElementById('major').value;
    const gpa = parseFloat(document.getElementById('gpa').value);
    const university = document.getElementById('university').value;
    const graduationYear = parseInt(document.getElementById('graduation-year').value);
    const country = document.getElementById('country').value;
    
    // Collect skills
    const skillsText = document.getElementById('skills').value;
    const skills = skillsText ? skillsText.split(',').map(s => s.trim()) : [];
    
    // Collect preferences
    const preferredCountriesText = document.getElementById('preferred-countries').value;
    const preferredFieldsText = document.getElementById('preferred-fields').value;
    const scholarshipType = document.getElementById('scholarship-type').value;
    const minAmount = document.getElementById('min-amount').value;
    
    const preferredCountries = preferredCountriesText ? preferredCountriesText.split(',').map(s => s.trim()) : [];
    const preferredFields = preferredFieldsText ? preferredFieldsText.split(',').map(s => s.trim()) : [];
    
    // Validate required fields
    if (!name || !email || !degreeLevel || !major || !gpa || !university || !graduationYear || !country) {
      alert('Please fill all required fields (marked sections)');
      return;
    }
    
    if (gpa < 0 || gpa > 4) {
      alert('GPA must be between 0 and 4.0');
      return;
    }
    
    updateStatus('🔄 Registering...', '#ffc107');
    
    // Create education object
    const education = {
      degree_level: { [degreeLevel]: null },
      major: major,
      university: university,
      gpa: gpa,
      graduation_year: graduationYear,
      country: country
    };
    
    // Create preferences object
    const preferences = {
      preferred_countries: preferredCountries,
      preferred_fields: preferredFields,
      scholarship_type: scholarshipType ? { [scholarshipType]: null } : { "FullScholarship": null },
      min_amount: minAmount ? [parseInt(minAmount)] : []
    };
    
    console.log('Registering user with:', { name, email, education, skills, preferences });
    
    // Call backend registration
    const response = await scholarship_backend.register_user(name, email, education, skills, preferences);
    
    console.log('Registration response:', response);
    
    if (response.includes('success') || response.includes('registered')) {
      alert('✅ Registration successful for: ' + name);
      document.getElementById('register-form').style.display = 'none';
      updateStatus('🟢 Registered', '#28a745');
      
      // Now load matches for the registered user
      setTimeout(() => {
        loadScholarshipsWithMatching();
      }, 1000);
      
    } else {
      alert('❌ Registration failed: ' + response);
      updateStatus('🔴 Registration Failed', '#dc3545');
    }
    
  } catch (error) {
    console.error('Registration error:', error);
    alert('❌ Registration error: ' + error.message);
    updateStatus('🔴 Registration Failed', '#dc3545');
  }
}

function updateStatus(text, color) {
  const status = document.getElementById('status');
  status.textContent = text;
  status.style.background = color;
}

function parseScholarshipResponse(response) {
  // If response doesn't contain scholarship data, return empty array
  if (!response || !response.includes('scholarships:')) {
    return [];
  }
  
  // Parse the text response into scholarship objects
  const lines = response.split('\n').filter(line => line.includes(' - '));
  const scholarships = [];
  
  lines.forEach((line, index) => {
    const parts = line.split(' - ');
    if (parts.length >= 3) {
      const nameAndId = parts[0].replace('- ', '').trim();
      const provider = parts[1] ? parts[1].replace('Provider: ', '').trim() : 'Unknown';
      const country = parts[2] ? parts[2].replace('Country: ', '').trim() : 'Unknown';
      
      scholarships.push({
        id: `scholarship-${index}`,
        name: nameAndId,
        provider: provider,
        country: country,
        funding_type: 'Full Scholarship',
        field_of_study: 'Various',
        description: 'Government sponsored scholarship program',
        degree_level: 'Master/PhD'
      });
    }
  });
  
  return scholarships;
}

function displayScholarships(scholarships) {
  // Handle empty or invalid data
  if (!Array.isArray(scholarships) || scholarships.length === 0) {
    document.getElementById('scholarships').innerHTML = `
      <div style="text-align: center; padding: 60px; color: #666; background: #f8f9fa; border-radius: 10px;">
        <h4>📚 No Scholarships Found</h4>
        <p>No scholarship data available at the moment.</p>
      </div>
    `;
    return;
  }

  let html = '';
  scholarships.forEach(scholarship => {
    html += `
      <div style="border: 1px solid #e9ecef; border-radius: 12px; padding: 20px; margin-bottom: 15px; background: #fff;">
        <h4 style="color: #007bff; margin: 0 0 10px 0;">${scholarship.name || 'Unknown Scholarship'}</h4>
        <p style="margin: 5px 0; color: #666;">🏛️ ${scholarship.provider || 'Unknown Provider'}</p>
        <p style="margin: 5px 0; color: #666;">🌍 ${scholarship.country || 'Unknown Country'}</p>
        <p style="margin: 5px 0; color: #666;">💰 ${scholarship.funding_type || 'Unknown Type'}</p>
        <p style="margin: 5px 0; color: #666;">📚 ${scholarship.field_of_study || 'Various Fields'}</p>
        <p style="margin: 10px 0; color: #333;">${scholarship.description || 'No description available'}</p>
        <span style="background: #007bff; color: white; padding: 6px 12px; border-radius: 15px; font-size: 14px;">
          🎓 ${scholarship.degree_level || 'All Levels'}
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
