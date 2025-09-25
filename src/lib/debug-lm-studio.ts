// Debug utility for LM Studio integration
export const debugLmStudio = {
  /**
   * Test LM Studio connection and log detailed information
   */
  async testConnection(baseUrl: string = 'http://localhost:1234') {
    console.group('🔧 LM Studio Debug - Connection Test');
    console.log('Testing URL:', baseUrl);
    
    try {
      const response = await fetch(`${baseUrl}/v1/models`);
      console.log('Response status:', response.status);
      console.log('Response headers:', Object.fromEntries(response.headers.entries()));
      
      if (!response.ok) {
        console.error('❌ Connection failed with status:', response.status);
        console.groupEnd();
        return false;
      }
      
      const data = await response.json();
      console.log('✅ Raw response data:', data);
      console.log('📊 Models found:', data.data?.length || 0);
      
      if (data.data) {
        data.data.forEach((model: any, index: number) => {
          console.log(`  ${index + 1}. ${model.id} (${model.object})`);
        });
      }
      
      console.groupEnd();
      return true;
    } catch (error) {
      console.error('❌ Connection error:', error);
      console.groupEnd();
      return false;
    }
  },

  /**
   * Test model fetching with detailed logging
   */
  async fetchModels(baseUrl: string = 'http://localhost:1234') {
    console.group('🤖 LM Studio Debug - Model Fetching');
    
    try {
      const connected = await this.testConnection(baseUrl);
      if (!connected) {
        console.error('❌ Cannot fetch models - connection failed');
        console.groupEnd();
        return [];
      }
      
      const response = await fetch(`${baseUrl}/v1/models`);
      const data = await response.json();
      
      const modelNames = data.data?.map((model: any) => model.id) || [];
      console.log('✅ Extracted model names:', modelNames);
      console.groupEnd();
      
      return modelNames;
    } catch (error) {
      console.error('❌ Model fetching error:', error);
      console.groupEnd();
      return [];
    }
  },

  /**
   * Check if LM Studio is running and accessible
   */
  async checkStatus(baseUrl: string = 'http://localhost:1234') {
    console.group('🏥 LM Studio Debug - Health Check');
    
    try {
      // Test basic connectivity
      const startTime = performance.now();
      const response = await fetch(`${baseUrl}/v1/models`, {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
        },
      });
      const endTime = performance.now();
      
      console.log(`⏱️  Response time: ${Math.round(endTime - startTime)}ms`);
      console.log(`🌐 Status: ${response.status} ${response.statusText}`);
      console.log(`📦 Content-Type: ${response.headers.get('content-type')}`);
      
      if (response.ok) {
        const data = await response.json();
        console.log('✅ LM Studio is running and responding');
        console.log(`🎯 API Version: ${data.object || 'unknown'}`);
        console.log(`📈 Models available: ${data.data?.length || 0}`);
      } else {
        console.log('⚠️  LM Studio responded but with error status');
      }
      
      console.groupEnd();
      return response.ok;
    } catch (error) {
      console.error('❌ LM Studio is not accessible:', error);
      console.log('💡 Make sure:');
      console.log('   • LM Studio is running');
      console.log('   • "Local server" is enabled in LM Studio');
      console.log('   • Server is running on the correct port');
      console.log('   • No firewall is blocking the connection');
      console.groupEnd();
      return false;
    }
  }
};

// Make it available globally for debugging
if (typeof window !== 'undefined') {
  (window as any).debugLmStudio = debugLmStudio;
}