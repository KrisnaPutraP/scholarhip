console.log('🎓 Scholarship Matcher Loading...');

// Import backend and authentication
import { scholarship_backend } from '../../declarations/scholarship_backend';
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory } from '../../declarations/scholarship_backend';

// Global auth client and actor
// Global variables
let authClient;
let authenticatedActor;
let isAuthenticated = false;

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
          <button id="viewAllBtn" style="margin: 5px; padding: 12px 20px; background: #28a745; color: white; border: none; border-radius: 8px; cursor: pointer;">
            📚 View All Scholarships
          </button>
          <button id="loginBtn" style="margin: 5px; padding: 12px 20px; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer;">
            🔐 Login with Internet Identity
          </button>
          <button id="testIIBtn" style="margin: 5px; padding: 12px 20px; background: #ffc107; color: black; border: none; border-radius: 8px; cursor: pointer;">
            🧪 Test Internet Identity
          </button>
          <button id="matchBtn" style="margin: 5px; padding: 12px 20px; background: #17a2b8; color: white; border: none; border-radius: 8px; cursor: pointer; display: none;">
            🎯 Get My Matches
          </button>
          <button id="profileBtn" style="margin: 5px; padding: 12px 20px; background: #6f42c1; color: white; border: none; border-radius: 8px; cursor: pointer; display: none;">
            👤 Check Profile
          </button>
          <button id="registerBtn" style="margin: 5px; padding: 12px 20px; background: #17a2b8; color: white; border: none; border-radius: 8px; cursor: pointer; display: none;">
            ✍️ Register Profile
          </button>
          <button id="logoutBtn" style="margin: 5px; padding: 12px 20px; background: #dc3545; color: white; border: none; border-radius: 8px; cursor: pointer; display: none;">
            🚪 Logout
          </button>
        </div>
      </header>

      <div style="background: white; padding: 30px; border-radius: 15px; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
        
        <div id="register-form" style="display: none; margin-bottom: 30px; padding: 20px; background: #f8f9fa; border-radius: 10px;">
          <h3>👤 User Registration</h3>
          
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;">
            <input type="text" id="name" placeholder="Full Name" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
            <input type="email" id="email" placeholder="Email" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px;">
          </div>
          
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
          
          <h4 style="margin: 20px 0 10px 0;">💪 Skills</h4>
          <input type="text" id="skills" placeholder="Skills (comma separated, e.g., Python, Research, English)" style="padding: 10px; border: 1px solid #ddd; border-radius: 6px; width: 100%; margin-bottom: 20px;">
          
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
              <p>Click "View All Scholarships" to browse opportunities</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  `;

  console.log('✅ UI created successfully!');
  
  // Initialize authentication
  initAuth();
  
  // Add event listeners
  document.getElementById('testBtn').addEventListener('click', testConnection);
  document.getElementById('viewAllBtn').addEventListener('click', viewAllScholarships);
  document.getElementById('loginBtn').addEventListener('click', login);
  document.getElementById('testIIBtn').addEventListener('click', testInternetIdentity);
  document.getElementById('matchBtn').addEventListener('click', loadScholarshipsWithMatching);
  document.getElementById('profileBtn').addEventListener('click', checkProfile);
  document.getElementById('registerBtn').addEventListener('click', showRegister);
  document.getElementById('submitRegBtn').addEventListener('click', registerUser);
  document.getElementById('logoutBtn').addEventListener('click', logout);
  
  console.log('✅ Event listeners attached!');
  
  // Auto-initialize and load scholarships on startup
  setTimeout(() => {
    autoInitializeAndLoad();
  }, 1000);
});

// ===========================================
// AUTHENTICATION FUNCTIONS WITH INTERNET IDENTITY
// ===========================================

async function initAuth() {
  try {
    console.log('🔐 Initializing Internet Identity authentication...');
    
    // Add error handling for AuthClient creation
    authClient = await AuthClient.create({
      idleOptions: {
        disableIdle: true,
        disableDefaultIdleCallback: true
      }
    });
    
    console.log('✅ AuthClient created successfully');
    
    const authenticated = await authClient.isAuthenticated();
    console.log('Authentication status:', authenticated);
    
    if (authenticated) {
      const identity = authClient.getIdentity();
      console.log('User principal:', identity.getPrincipal().toString());
      await handleAuthenticated();
    } else {
      handleNotAuthenticated();
    }
  } catch (error) {
    console.error('❌ Error initializing auth:', error);
    handleNotAuthenticated();
    // Show user-friendly error
    updateStatus('🔴 Auth Init Failed', '#dc3545');
  }
}

async function login() {
  try {
    console.log('🔐 Starting Internet Identity login...');
    updateStatus('🔄 Connecting to Internet Identity...', '#ffc107');
    
    // Use the working URL from your screenshot
    const identityProviderUrl = `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`;
    
    console.log('Using Identity Provider URL:', identityProviderUrl);
    
    await authClient.login({
      identityProvider: identityProviderUrl,
      maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 7 days in nanoseconds
      windowOpenerFeatures: "toolbar=0,location=0,menubar=0,width=500,height=600,left=100,top=100",
      onSuccess: async () => {
        console.log('✅ Login successful');
        const identity = authClient.getIdentity();
        console.log('User principal:', identity.getPrincipal().toString());
        await handleAuthenticated();
        alert('🎉 Successfully logged in with Internet Identity!\n\nPrincipal: ' + identity.getPrincipal().toString());
      },
      onError: (error) => {
        console.error('❌ Login failed:', error);
        updateStatus('🔴 Login Failed', '#dc3545');
        alert(`❌ Login failed: ${error}\n\nSilakan coba lagi atau pilih Identity yang berbeda.`);
      }
    });
  } catch (error) {
    console.error('Error during login:', error);
    updateStatus('🔴 Login Error', '#dc3545');
    alert('❌ Error during login: ' + error.message);
  }
}

async function logout() {
  try {
    console.log('🚪 Logging out...');
    updateStatus('🔄 Logging out...', '#ffc107');
    
    await authClient.logout();
    
    // Clear any local storage
    localStorage.removeItem('currentUser');
    
    handleNotAuthenticated();
    
    // Clear scholarships display
    document.getElementById('scholarships').innerHTML = 
      '<div style="text-align: center; padding: 60px; color: #666; background: #f8f9fa; border-radius: 10px;">' +
        '<h4>👋 You have been logged out</h4>' +
        '<p>Login with Internet Identity to see scholarship matches</p>' +
      '</div>';
    
    alert('👋 You have been logged out successfully!');
    console.log('✅ Logout complete');
    
  } catch (error) {
    console.error('Error during logout:', error);
    alert('❌ Error during logout: ' + error.message);
  }
}

async function handleAuthenticated() {
  isAuthenticated = true;
  updateStatus('🟢 Authenticated', '#28a745');
  
  // Create authenticated actor
  await createAuthenticatedActor();
  
  // Clear registration form when switching Internet Identity accounts
  clearRegistrationForm();
  
  // Check if user is already registered
  const isUserRegistered = await checkIfUserRegistered();
  
  // Show authenticated user buttons
  document.getElementById('loginBtn').style.display = 'none';
  document.getElementById('matchBtn').style.display = 'inline-block';
  document.getElementById('profileBtn').style.display = 'inline-block';
  document.getElementById('logoutBtn').style.display = 'inline-block';
  
  // Only show register button if user is not already registered
  if (isUserRegistered) {
    document.getElementById('registerBtn').style.display = 'none';
    console.log('🚫 Register button hidden - user already registered');
  } else {
    document.getElementById('registerBtn').style.display = 'inline-block';
    console.log('✅ Register button shown - user not registered');
  }
  
  console.log('✅ User authenticated, UI updated, registration form cleared');
}

async function createAuthenticatedActor() {
  try {
    const identity = authClient.getIdentity();
    
    // Create HTTP agent with the authenticated identity
    const agent = new HttpAgent({
      host: 'http://localhost:4943',
      identity: identity,
    });
    
    // In development, we need to fetch the root key
    await agent.fetchRootKey();
    
    // Create authenticated actor
    authenticatedActor = Actor.createActor(idlFactory, {
      agent,
      canisterId: 'bkyz2-fmaaa-aaaaa-qaaaq-cai',
    });
    
    console.log('✅ Authenticated actor created with principal:', identity.getPrincipal().toString());
  } catch (error) {
    console.error('❌ Error creating authenticated actor:', error);
  }
}

function handleNotAuthenticated() {
  isAuthenticated = false;
  updateStatus('🔐 Not Authenticated', '#6c757d');
  
  // Show login button, hide authenticated buttons
  document.getElementById('loginBtn').style.display = 'inline-block';
  document.getElementById('matchBtn').style.display = 'none';
  document.getElementById('profileBtn').style.display = 'none';
  document.getElementById('registerBtn').style.display = 'none';
  document.getElementById('logoutBtn').style.display = 'none';
  
    console.log('User not authenticated, UI updated');
}

// Test Internet Identity function
function testInternetIdentity() {
  const urls = [
    `http://127.0.0.1:4943/?canisterId=rdmx6-jaaaa-aaaaa-aaadq-cai`,
    `http://127.0.0.1:4943/?canisterId=be2us-64aaa-aaaaa-qaabq-cai&id=rdmx6-jaaaa-aaaaa-aaadq-cai`,
    `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`
  ];
  
  console.log('🧪 Testing Internet Identity URLs...');
  updateStatus('🧪 Testing Internet Identity...', '#ffc107');
  
  urls.forEach((url, index) => {
    setTimeout(() => {
      console.log(`Testing URL ${index + 1}: ${url}`);
      const popup = window.open(url, `test_${index}`, 'width=500,height=600');
      
      setTimeout(() => {
        if (popup && !popup.closed) {
          console.log(`✅ URL ${index + 1} works: ${url}`);
          popup.close();
        } else {
          console.log(`❌ URL ${index + 1} failed: ${url}`);
        }
      }, 2000);
    }, index * 500);
  });
  
  alert('🧪 Testing Internet Identity URLs. Check console for results.');
}

// ===========================================
// UTILITY FUNCTIONS
// ===========================================

// ===========================================
// SESSION MANAGEMENT FUNCTIONS (LEGACY - keeping for compatibility)
// ===========================================

// Session Management Functions
function saveUserSession(userInfo) {
  localStorage.setItem('currentUser', JSON.stringify(userInfo));
  updateUIForLoggedInUser(userInfo);
}

function getCurrentUser() {
  const userStr = localStorage.getItem('currentUser');
  return userStr ? JSON.parse(userStr) : null;
}

function updateUIForLoggedInUser(userInfo) {
  document.getElementById('logoutBtn').style.display = 'inline-block';
  document.getElementById('registerBtn').style.display = 'none';
  updateStatus('👤 Logged in as: ' + userInfo.name, '#28a745');
}

function updateUIForLoggedOutUser() {
  document.getElementById('logoutBtn').style.display = 'none';
  document.getElementById('registerBtn').style.display = 'inline-block';
  updateStatus('✅ Frontend Active', '#6c757d');
}

// ===========================================
// SCHOLARSHIP FUNCTIONS
// ===========================================

// View All Scholarships Function
async function viewAllScholarships() {
  try {
    updateStatus('🔄 Loading All Scholarships...', '#ffc107');
    console.log('Loading all scholarships from backend...');
    
    const response = await scholarship_backend.get_all_scholarships();
    console.log('All scholarships response:', response);
    
    const scholarships = parseScholarshipResponse(response);
    
    if (scholarships && scholarships.length > 0) {
      displayScholarships(scholarships, "All Available Scholarships", false);
      updateStatus('📚 All Scholarships Loaded', '#28a745');
    } else {
      document.getElementById('scholarships').innerHTML = 
        '<div style="text-align: center; padding: 40px; color: #666;">' +
          '<h4>📚 No scholarships available</h4>' +
          '<p>Please check back later or contact support.</p>' +
        '</div>';
      updateStatus('📚 No Scholarships Found', '#ffc107');
    }
    
  } catch (error) {
    console.error('Error loading all scholarships:', error);
    alert('❌ Error loading scholarships: ' + error.message);
    updateStatus('🔴 Error Loading Scholarships', '#dc3545');
  }
}

// Auto-initialization function
async function autoInitializeAndLoad() {
  try {
    console.log('🚀 Auto-initializing scholarship system...');
    updateStatus('🔄 Starting System...', '#ffc107');
    
    // Note: Authentication status is handled by initAuth()
    // We don't need to check localStorage for user sessions anymore
    
    console.log('Step 1: Testing backend connection...');
    const connectionTest = await scholarship_backend.test_connection();
    console.log('Connection test result:', connectionTest);
    
    if (!connectionTest || connectionTest.includes('Error')) {
      throw new Error('Backend connection failed');
    }
    
    console.log('Step 2: Checking existing scholarships...');
    const existingScholarships = await scholarship_backend.get_all_scholarships();
    console.log('Existing scholarships check:', existingScholarships);
    
    if (!existingScholarships || existingScholarships.includes('No scholarships found') || existingScholarships.trim() === '') {
      console.log('Step 3: No scholarships found, initializing sample data...');
      updateStatus('🔄 Initializing Data...', '#ffc107');
      
      const initResult = await scholarship_backend.manual_init_scholarships();
      console.log('Initialization result:', initResult);
      
      if (!initResult || initResult.includes('Error')) {
        throw new Error('Failed to initialize sample data');
      }
    }
    
    console.log('Step 4: Loading scholarships...');
    updateStatus('🔄 Loading Scholarships...', '#28a745');
    
    const scholarshipsResponse = await scholarship_backend.get_all_scholarships();
    const scholarships = parseScholarshipResponse(scholarshipsResponse);
    
    if (scholarships && scholarships.length > 0) {
      displayScholarships(scholarships, 'Available Scholarships', false);
      updateStatus('🟢 Ready (' + scholarships.length + ' scholarships)', '#28a745');
      console.log('✅ Auto-initialization complete! Loaded ' + scholarships.length + ' scholarships.');
      
      showAutoLoadSuccess(scholarships.length);
    } else {
      throw new Error('No scholarships could be loaded');
    }
    
  } catch (error) {
    console.error('❌ Auto-initialization failed:', error);
    updateStatus('🔴 System Error', '#dc3545');
    showErrorFallback(error.message);
  }
}

function showAutoLoadSuccess(count) {
  const banner = document.createElement('div');
  banner.style.cssText = 
    'position: fixed; top: 20px; right: 20px; background: linear-gradient(135deg, #28a745, #20c997); ' +
    'color: white; padding: 15px 20px; border-radius: 10px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); ' +
    'z-index: 1000; font-weight: bold;';
  
  banner.innerHTML = 
    '<div style="display: flex; align-items: center;">' +
      '<span style="font-size: 20px; margin-right: 10px;">✅</span>' +
      '<div>' +
        '<div>System Ready!</div>' +
        '<div style="font-size: 12px; opacity: 0.9;">' + count + ' scholarships loaded automatically</div>' +
      '</div>' +
    '</div>';
  
  document.body.appendChild(banner);
  
  setTimeout(() => {
    if (banner.parentNode) {
      banner.parentNode.removeChild(banner);
    }
  }, 4000);
}

function showErrorFallback(errorMessage) {
  document.getElementById('scholarships').innerHTML = 
    '<div style="text-align: center; padding: 40px; background: #f8d7da; border: 2px solid #dc3545; border-radius: 15px; color: #721c24;">' +
      '<h3 style="color: #dc3545; margin: 0 0 20px 0;">⚠️ System Initialization Failed</h3>' +
      '<div style="background: white; padding: 20px; border-radius: 10px; margin: 20px 0; text-align: left;">' +
        '<h4 style="color: #dc3545; margin: 0 0 10px 0;">Error Details:</h4>' +
        '<p style="margin: 0; font-family: monospace; color: #721c24; word-break: break-word;">' + errorMessage + '</p>' +
      '</div>' +
    '</div>';
}

// Functions
async function testConnection() {
  try {
    console.log('Testing connection...');
    updateStatus('🔄 Testing...', '#ffc107');
    
    const response = await scholarship_backend.test_connection();
    
    alert('✅ Backend connected! Response: ' + JSON.stringify(response));
    updateStatus('🟢 Connected', '#28a745');
  } catch (error) {
    console.error('Backend error:', error);
    updateStatus('🔴 Backend Failed', '#dc3545');
    alert('❌ Error: ' + error.message);
  }
}

async function checkProfile() {
  // Check authentication first
  if (!isAuthenticated) {
    alert('🔐 Please login with Internet Identity first!');
    return;
  }
  
  if (!authenticatedActor) {
    console.error('❌ No authenticated actor available');
    alert('Please login first');
    return;
  }

  // Close registration form if it's open
  document.getElementById('register-form').style.display = 'none';
  console.log('🗂️ Registration form closed (from Check Profile)');
  
  try {
    console.log('Checking user profile...');
    updateStatus('🔄 Checking Profile...', '#ffc107');
    
    // Debug: Log current principal
    const identity = authClient.getIdentity();
    const principal = identity.getPrincipal().toString();
    console.log('🔍 Current Principal ID:', principal);
    
    const profile = await authenticatedActor.get_my_profile();
    console.log('📄 Profile response:', profile);
    
    if (profile.includes('error') || profile.includes('not found')) {
      alert(`❌ No user profile found for Principal: ${principal}\n\nResponse: ${profile}\n\nPlease register first.`);
      updateStatus('🔴 Not Registered', '#dc3545');
    } else {
      alert(`✅ User profile found for Principal: ${principal}\n\nProfile: ${profile}`);
      updateStatus('🟢 Profile Found', '#28a745');
    }
  } catch (error) {
    console.error('Profile check error:', error);
    alert('❌ Error checking profile: ' + error.message);
    updateStatus('🔴 Profile Check Failed', '#dc3545');
  }
}

async function loadScholarshipsWithMatching() {
  // Check authentication first
  if (!isAuthenticated) {
    alert('🔐 Please login with Internet Identity first!');
    return;
  }
  
  if (!authenticatedActor) {
    console.error('❌ No authenticated actor available');
    alert('Please login first');
    return;
  }

  // Close registration form if it's open
  document.getElementById('register-form').style.display = 'none';
  console.log('🗂️ Registration form closed (from Get My Matches)');
  
  try {
    console.log('Loading scholarships with matching...');
    updateStatus('🔄 Loading Matches...', '#ffc107');
    
    console.log('Calling get_my_matches...');
    let recommendations = await authenticatedActor.get_my_matches();
    console.log('get_my_matches response:', recommendations);
    
    if (!recommendations || recommendations.includes('Error:') || recommendations.includes('not found')) {
      console.log('Trying get_my_recommendations with limit...');
      recommendations = await authenticatedActor.get_my_recommendations([10]);
      console.log('get_my_recommendations response:', recommendations);
    }
    
    console.log('Final recommendations:', recommendations);
    
    if (recommendations && !recommendations.includes('Error:') && !recommendations.includes('not found')) {
      console.log('Processing valid recommendations...');
      parseAndDisplayRecommendations(recommendations);
      updateStatus('🟢 Matches Loaded', '#28a745');
    } else {
      console.log('No valid recommendations, error:', recommendations);
      alert('⚠️ Backend response: ' + recommendations + '\n\nPlease make sure you are registered first.');
      
      console.log('Loading regular scholarships as fallback...');
      loadScholarships();
    }
  } catch (error) {
    console.error('Error loading matches:', error);
    alert('❌ Error getting matches: ' + error.message + '\n\nFalling back to regular scholarships.');
    loadScholarships();
  }
}

function parseAndDisplayRecommendations(response) {
  console.log('Parsing recommendations:', response);
  
  if (response.includes('Error:') || response.includes('not found')) {
    alert('⚠️ ' + response + '\nPlease register first to get personalized matches.');
    loadScholarships();
    return;
  }
  
  const text = response.replace(/🎯/g, '').trim();
  const scholarshipMatches = [];
  const parts = text.split(',');
  
  parts.forEach(part => {
    const trimmed = part.trim();
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
  
  scholarshipMatches.sort((a, b) => b.percentage - a.percentage);
  
  let html = 
    '<div style="text-align: center; margin-bottom: 30px;">' +
      '<h3 style="color: #007bff; margin: 0 0 10px 0;">🎯 Your Personalized Matches</h3>' +
      '<p style="color: #666; margin: 0 0 20px 0;">AI-powered recommendations based on your profile</p>' +
    '</div>';
  
  scholarshipMatches.forEach((scholarship, index) => {
    const percentage = scholarship.percentage;
    const name = scholarship.name;
    
    let bgColor, badgeColor, emoji;
    if (percentage >= 75) {
      bgColor = '#d4edda';
      badgeColor = '#28a745';
      emoji = '🏆';
    } else if (percentage >= 65) {
      bgColor = '#fff3cd';
      badgeColor = '#ffc107';
      emoji = '🥈';
    } else {
      bgColor = '#f8d7da';
      badgeColor = '#6c757d';
      emoji = '📋';
    }
    
    html += 
      '<div style="border: 2px solid ' + badgeColor + '; border-radius: 15px; padding: 25px; margin-bottom: 20px; background: ' + bgColor + ';">' +
        '<div style="display: flex; justify-content: space-between; align-items: center;">' +
          '<div style="display: flex; align-items: center;">' +
            '<span style="font-size: 24px; margin-right: 10px;">' + emoji + '</span>' +
            '<h4 style="color: #333; margin: 0;">' + name + '</h4>' +
          '</div>' +
          '<div style="background: ' + badgeColor + '; color: white; padding: 10px 15px; border-radius: 25px; font-weight: bold;">' +
            percentage.toFixed(1) + '%' +
          '</div>' +
        '</div>' +
      '</div>';
  });
  
  document.getElementById('scholarships').innerHTML = html;
  updateStatus('🎯 Matches Found', '#28a745');
}

async function loadScholarships() {
  try {
    console.log('Loading scholarships...');
    updateStatus('🔄 Loading...', '#ffc107');
    
    const response = await scholarship_backend.get_all_scholarships();
    const scholarships = parseScholarshipResponse(response);
    
    console.log('Loaded scholarships:', scholarships);
    displayScholarships(scholarships, 'Available Scholarships', false);
    updateStatus('🟢 Loaded from IC', '#28a745');
    alert('✅ ' + scholarships.length + ' scholarships loaded from IC backend!');
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
  // Check authentication first
  if (!isAuthenticated) {
    alert('🔐 Please login with Internet Identity first!');
    return;
  }
  
  if (!authenticatedActor) {
    console.error('❌ No authenticated actor available');
    alert('Please login first');
    return;
  }
  
  try {
    const name = document.getElementById('name').value;
    const email = document.getElementById('email').value;
    
    const degreeLevel = document.getElementById('degree-level').value;
    const major = document.getElementById('major').value;
    const gpa = parseFloat(document.getElementById('gpa').value);
    const university = document.getElementById('university').value;
    const graduationYear = parseInt(document.getElementById('graduation-year').value);
    const country = document.getElementById('country').value;
    
    const skillsText = document.getElementById('skills').value;
    const skills = skillsText ? skillsText.split(',').map(s => s.trim()) : [];
    
    const preferredCountriesText = document.getElementById('preferred-countries').value;
    const preferredCountries = preferredCountriesText ? preferredCountriesText.split(',').map(s => s.trim()) : [];
    
    const preferredFieldsText = document.getElementById('preferred-fields').value;
    const preferredFields = preferredFieldsText ? preferredFieldsText.split(',').map(s => s.trim()) : [];
    
    const scholarshipType = document.getElementById('scholarship-type').value;
    const minAmount = document.getElementById('min-amount').value ? parseInt(document.getElementById('min-amount').value) : null;
    
    if (!name || !email || !degreeLevel || !major || !gpa || !university || !graduationYear || !country) {
      alert('❌ Please fill in all required fields!');
      return;
    }
    
    updateStatus('🔄 Registering...', '#ffc107');
    
    // Debug: Log current principal during registration
    const identity = authClient.getIdentity();
    const principal = identity.getPrincipal().toString();
    console.log('🔍 Registering with Principal ID:', principal);
    
    const education = {
      degree_level: { [degreeLevel]: null },
      major: major,
      gpa: gpa,
      university: university,
      graduation_year: graduationYear,
      country: country
    };
    
    const preferences = {
      preferred_countries: preferredCountries,
      preferred_fields: preferredFields,
      scholarship_type: scholarshipType ? { [scholarshipType]: null } : { 'FullScholarship': null },
      min_amount: minAmount ? [minAmount] : []
    };
    
    console.log('Registering user with data:', { name, email, education, skills, preferences });
    console.log('🔍 Principal during registration:', principal);
    
    const response = await authenticatedActor.register_user(name, email, education, skills, preferences);
    
    console.log('Registration response:', response);
    
    if (response.includes('successfully') || response.includes('Success')) {
      alert('✅ Registration successful! Welcome ' + name + '!');
      updateStatus('🟢 Registration Complete', '#28a745');
      
      // Hide register button since user is now registered
      document.getElementById('registerBtn').style.display = 'none';
      console.log('🚫 Register button hidden - user successfully registered');
      
      saveUserSession({ name, email });
      
      document.getElementById('register-form').style.display = 'none';
      
      setTimeout(() => {
        loadScholarshipsWithMatching();
      }, 1000);
    } else {
      throw new Error(response);
    }
    
  } catch (error) {
    console.error('Registration error:', error);
    alert('❌ Registration failed: ' + error.message);
    updateStatus('🔴 Registration Failed', '#dc3545');
  }
}

function updateStatus(text, color) {
  const status = document.getElementById('status');
  status.textContent = text;
  status.style.background = color;
}

function clearRegistrationForm() {
  // Clear all registration form fields
  const form = document.querySelector('form');
  if (form) {
    form.reset();
  }
  
  // Also manually clear specific fields to ensure they're empty
  const fields = ['name', 'email', 'university', 'degree', 'gpa', 'interests', 'achievements'];
  fields.forEach(fieldId => {
    const field = document.getElementById(fieldId);
    if (field) {
      field.value = '';
    }
  });
  
  console.log('🧹 Registration form cleared for new user');
}

async function checkIfUserRegistered() {
  if (!authenticatedActor) {
    console.log('❌ No authenticated actor available for registration check');
    return false;
  }
  
  try {
    const profile = await authenticatedActor.get_my_profile();
    const isRegistered = !profile.includes('error') && !profile.includes('not found');
    console.log('🔍 User registration status:', isRegistered ? 'Registered' : 'Not registered');
    return isRegistered;
  } catch (error) {
    console.error('❌ Error checking registration status:', error);
    return false;
  }
}

function parseScholarshipResponse(response) {
  if (!response || !response.includes('scholarships:')) {
    return [];
  }
  
  const lines = response.split('\n').filter(line => line.includes(' - '));
  const scholarships = [];
  
  lines.forEach((line, index) => {
    const parts = line.split(' - ');
    if (parts.length >= 3) {
      const nameAndId = parts[0].replace('- ', '').trim();
      const provider = parts[1] ? parts[1].replace('Provider: ', '').trim() : 'Unknown';
      const country = parts[2] ? parts[2].replace('Country: ', '').trim() : 'Unknown';
      
      scholarships.push({
        id: 'scholarship-' + index,
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

function displayScholarships(scholarships, title, showMatching) {
  if (!scholarships || scholarships.length === 0) {
    document.getElementById('scholarships').innerHTML = 
      '<div style="text-align: center; padding: 40px; color: #666;">' +
        '<h4>📚 No scholarships available</h4>' +
        '<p>Please check back later or contact support.</p>' +
      '</div>';
    return;
  }

  let html = 
    '<div style="text-align: center; margin-bottom: 30px;">' +
      '<h3 style="color: #007bff; margin: 0 0 10px 0;">' + title + '</h3>' +
      '<p style="color: #666; margin: 0;">Found ' + scholarships.length + ' available scholarships</p>' +
    '</div>';

  scholarships.forEach(scholarship => {
    html += 
      '<div style="border: 1px solid #ddd; border-radius: 10px; padding: 20px; margin-bottom: 15px; background: white; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">' +
        '<h4 style="color: #333; margin: 0 0 10px 0;">' + scholarship.name + '</h4>' +
        '<div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px; color: #666; font-size: 14px;">' +
          '<p><strong>Provider:</strong> ' + scholarship.provider + '</p>' +
          '<p><strong>Country:</strong> ' + scholarship.country + '</p>' +
          '<p><strong>Type:</strong> ' + scholarship.funding_type + '</p>' +
          '<p><strong>Field:</strong> ' + scholarship.field_of_study + '</p>' +
        '</div>' +
        '<p style="margin: 10px 0 0 0; color: #555;">' + scholarship.description + '</p>' +
      '</div>';
  });

  document.getElementById('scholarships').innerHTML = html;
}

function displaySample() {
  const sampleScholarships = [
    {
      name: "Global Excellence Scholarship 2025",
      provider: "International Education Foundation",
      country: "Multiple Countries",
      funding_type: "Full Scholarship",
      field_of_study: "All Fields",
      description: "Comprehensive scholarship covering tuition, living expenses, and research funding."
    },
    {
      name: "STEM Innovation Grant",
      provider: "Tech Future Institute",
      country: "USA",
      funding_type: "Partial Scholarship",
      field_of_study: "Science & Technology",
      description: "Supporting outstanding students in STEM fields with partial funding and mentorship."
    }
  ];
  
  displayScholarships(sampleScholarships, "Sample Scholarships", false);
}