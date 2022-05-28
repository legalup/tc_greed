#define a2z(v)            (v).begin(),(v).end()
#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define len(v)   (v).size()

class SparseVector {
public:
    
    SparseVector(vector<int> &nums){
      int sz = nums.size();
      fi(0,sz) if(nums[i]) {
          ns[i]=nums[i];
        keys.insert(i);
      }
    }
    
    // Return the dotProduct of two sparse vectors
    int dotProduct(SparseVector& vec) {
      vector<int> output(keys.size());
      auto sit = set_intersection(a2z(keys),a2z(vec.keys),output.begin());
        
        auto dash_fold = [&](int a, int b) {
                         return a+vec.ns[b]*ns[b];
                     };
      return std::accumulate(output.begin(), sit, 0,dash_fold);

      
    }

  unordered_map<int,int> ns;
  set<int> keys;
};

// Your SparseVector object will be instantiated and called as such:
// SparseVector v1(nums1);
// SparseVector v2(nums2);
// int ans = v1.dotProduct(v2);
