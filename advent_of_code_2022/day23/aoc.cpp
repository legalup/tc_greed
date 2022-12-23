
#include <bits/stdc++.h>
using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fori(a) for(int i=((int)(0)); i < ((int)(a.size())); i++)
#define forj(a) for(int j=((int)(0)); j < ((int)(a.size())); j++)
#define a2z(v)            (v).begin(),(v).end()
#define isin(b,a)        find(all(a),(b))!=(a).end()
#define isnotin(b,a)        find(all(a),(b))==(a).end()
#define len(v)   (v).size()
#define forzin(acont) for(auto z : acont)

using vi=vector<int>;
using si=set<int>;
using vd=vector<double>;
using graph = map<int,multiset<int>>;

using ull=long long;
using pii = pair<int,int>;
using tri=tuple<int,int,int>;
using pq = priority_queue<int, vector<int>, less<int>>;


namespace std
{
  std::ostream& operator<<(std::ostream& strm,
                           const std::pair<int,int>& kvPair)
    {
      strm << "(" << kvPair.first << "," << kvPair.second << ")";
      return strm;
    }
}


template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

template <class T>
void printd(deque<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}


template <typename T, class Container = vector<T>,
        class Compare = less<typename Container::value_type>>
class priority_queue_with_remove : public priority_queue<T, Container, Compare> {
public:
    bool remove(const T &valueToRemove) {
        auto itr = std::find(this->c.begin(), this->c.end(), valueToRemove);
        if (itr == this->c.end()) {
            return false;
        }
        this->c.erase(itr);
        // ideally we should not be rebuilding the heap as we are removing only one element
        // a custom implementation of priority queue can adjust only one element in O(logN)
        std::make_heap(this->c.begin(), this->c.end(), this->comp);
        return true;
    }
};

struct gtpii
{
    bool operator()(const pii & p1, const pii & p2)
    {
        auto & [a1,idx1] = p1;
        auto & [a2,idx2] = p2;
        if(a1 != a2) return a1>a2;
        return idx1 > idx2;
    }
};

using pqpii = priority_queue<pii, vector<pii>, gtpii>;

void
split( vector<string> & theStringVector,  /* Altered/returned value */
       const  string  & theString,
       const  string & theDelimiters)
{
  assert(!theDelimiters.empty());
  char buffy[theString.size()];
  strcpy(buffy,theString.c_str());

  char  * str1 = strtok(buffy,theDelimiters.c_str());
  theStringVector.push_back(str1);

  while((str1 = strtok(0,theDelimiters.c_str()))) theStringVector.push_back(str1);
  return;

}

//////////////////////////////////
// start here//
//////////////////////////

int print(map<pii,char> brd)
{
  int maxr=-1000000;
  int minr = 1000000;
  int maxc = -1000000;
  int minc = 1000000;
  for(auto pp : brd){
    maxr = max(maxr,pp.first.first);
    minr = min(minr,pp.first.first);
    maxc = max(maxc,pp.first.second);
    minc = min(minc,pp.first.second);
  }

  int nrows = maxr-minr +1;
  int ncols = maxc - minc +1;

  string zrow;
  fi(0,ncols) zrow.push_back('.');
  vector<string> board(nrows,zrow);

  for(auto pp : brd){
    board[pp.first.first - minr][pp.first.second-minc] = '#';
  }

  forzin(board) cout << z << endl;
  int sum(0);

  fi(0,nrows)fj(0,ncols) if(board[i][j] == '.') sum++;
  return sum;
}

void print(map<pii,int> prop)
{

  for(auto pp : prop){
    cout << pp.first << "," << pp.second << endl;
  }
}

struct node
{
  node(int r, int c): row(r),col(c)
  {};

  node() = default;
  ~node() = default;

  int row;
  int col;

  bool isok(map<pii, char>& brd)
  {
    fi(-1,2)fj(-1,2){
      int nr = row + i;
      int nc = col + j;
      if(i==0 && j==0) continue;
      auto pp = make_pair(nr,nc);
      if(brd.count(pp)==0) continue;
      if(brd[pp] == '#') return false;
    }

    return true;
    
  };
  
  int proposal=-1;

  bool canpropose(int dir, map<pii,char> & brd) const
  {

    vector<pii> tocheck;
                       
    if(dir == 0){
      fi(-1,2) tocheck.push_back({row-1,col+i});
    }
    else if(dir == 1){
      fi(-1,2) tocheck.push_back({row+1,col+i});
    }
    else if(dir == 2){
      fi(-1,2) tocheck.push_back({row+i,col-1});
    }
    else{
      fi(-1,2) tocheck.push_back({row+i,col+1});
    }

    forzin(tocheck) if(brd.count(z)>0) return false;

    return true;

  }

  //figures out the proposal. if happy where it is,
  // proposal == -1. otherwise 0 north, 1 south, 2 west,3 east
  // saves it off to proposal, and adds 1 to positions
  void make_proposal(map<pii,int> & prop, map<pii,char> & board, int round)
  {
    proposal = -1;
    if(isok(board)) return;

    fi(0,4){
      int idx = (round+i) % 4;

      if(canpropose(idx,board)){
        proposal = idx;

        pii toput;
        if(idx==0)  toput = {row-1,col};
        if(idx==1)  toput = {row+1,col};
        if(idx==2)  toput = {row,col-1};
        if(idx==3)  toput = {row,col+1};

        if(prop.count(toput)>0) prop[toput] += 1;
        else prop[toput] = 1;
        return;

      }
    }
    
  };

  //updates this board on its lcoation, if it can move
  void move(map<pii,int> &prop, map<pii,char> & board)
  {
    if(proposal == -1) return;

    pii toput;

    if(proposal==0) toput = {row-1,col};
    if(proposal==1) toput = {row+1,col};
    if(proposal==2) toput = {row,col-1};
    if(proposal==3) toput = {row,col+1};


    assert(prop.count(toput)>0);
    pii curry = {row,col};
                
    if(prop[toput] < 2){
      board.erase(curry);
      board[toput] = '#';
      row = toput.first;
      col = toput.second;
      assert(board.count(curry) == 0);
    }
  }
  
};



template<typename T>
void run(T & cc){

  vector<string> lines;
  for (std::string line; std::getline(cc, line); ) 
  {
    if(!line.empty())
    lines.push_back(line);
  }

  int nrows = len(lines);
  int ncols = len(lines[0]);

  map<pii,char> board;
  // ledts make our nodes, shall we
  vector<node *> nodes;
  fi(0,nrows)fj(0,ncols){
    if(lines[i][j] == '#'){
      nodes.push_back(new node(i,j));
      board[{i,j}] = '#';
    }
  };

  //verify
  //printf("nrows: %d and ncols:%d\n",nrows,ncols);
  forzin(nodes) printf("node  at (%d, %d)\n",z->row, z->col);
  //print(board);

  //char ch;
  //cin >> ch;

  auto iswehappy
      = [&]()->bool{
          forzin(nodes) if(!z->isok(board)) return false;

          return true;
        };
                   
  cout << "starting" << endl;
 print(board);
    char ch;
    cin >> ch;
    int round=0;
  for(; ; round++)
  {
    
    map<pii,int> props;
    forzin(nodes) z->make_proposal(props,board, round);

    // print(props);
    //  char ch;
    //  cin >> ch;
    

    forzin(nodes) z->move(props, board);

    // print(board);

    // cin >> ch;

    if(iswehappy()) break;
    
  }

  cout << "this is the final board " << endl;
  int ret = print(board);

  cout << " the answer is: " << ret << endl;

  cout << "and round is: " << round << endl;

};

int
main(int argc, char**argv)
{

  ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
   
 #define TEST

#ifdef TEST
  ifstream myfile ("data.txt");
  run(myfile);

  
#else
  run(cin);
#endif
  
}
 
