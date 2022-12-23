
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

struct node{
  node(int r, int c):row(r),col(c)
  {}

  node() = default;
  ~node() = default;
  int row=-1;
  int col = -1;

  node * next(int dir){
    return nbrs[dir];
  }

  int adddir(int dir, int i){

    dir = (dir +i) % 4;
    if(dir<0) dir += 4;
    return dir;
  }

  vector<node*> nbrs = vector<node*>(4,nullptr);
};

void print(vector<string> bd ){
  forzin(bd) cout << z << endl;
   char x;
    cin >> x; 
}

node * move(pii command , node * curr, int & dir, vector<string> & board)
{
  assert(curr != nullptr);
  auto [m,d] = command;

  for(int i=0; i<m; i++){
    if(curr->next(dir) != nullptr){
      curr = curr->next(dir);
      char & ch = board[curr->row][curr->col];
      if(dir == 0) ch = '>';
      else if(dir == 1) ch = 'V';
      else if(dir == 2) ch = '<';
      else if(dir == 3) ch = 'A';
      cout << "top print " << i << endl;
      //print(board);
    }
  };

  if( !(d == 1 || d == -1 || d == 0)){
    cout << "wtf??? dir is: " << d << endl;
    cout << " m is: " << m << endl;
  };
  dir = curr->adddir(dir,d);
 char & ch = board[curr->row][curr->col];
      if(dir == 0) ch = '>';
      else if(dir == 1) ch = 'V';
      else if(dir == 2) ch = '<';
      else if(dir == 3) ch = 'A';
            cout << "bottom print" << endl;
            //print(board);
  return curr;
}




template<typename T>
void run(T & cc){

  vector<string> lines;
  for (std::string line; std::getline(cc, line); ) 
  {
    if(!line.empty())
    lines.push_back(line);
  }

  vector<string> board;
  

  size_t nrows = len(lines)-1;
  fi(0,nrows) board.push_back(lines[i]);
  size_t ncols = len(lines[0]);
  for(auto line : board) ncols = max(ncols,len(line));

  fi(0,nrows){
    while(len(board[i])<ncols) board[i].push_back(' ');
  }


  cout << "nrows, ncols:" << nrows << "," << ncols << endl;
  vector<vector<node*>> grid;
  fi(0,nrows){
    vector<node*> arow(ncols,nullptr);
    grid.push_back(arow);
  }
  cout << "hi1 " << endl;
  //first instantiate what you need
  fi(0,nrows)fj(0,len(board[i])){
    if(board[i][j] == '.') grid[i][j] = new node(i,j);
  }
  
  fi(0,nrows)fj(0,ncols){

    node * curr = grid[i][j];
    if(curr == nullptr) continue;
    if(i>0) curr->nbrs[3] = grid[i-1][j];
    if(j>0) curr->nbrs[2] = grid[i][j-1];
    if(i<nrows-1) curr->nbrs[1] = grid[i+1][j];
    if(j<ncols-1 ) curr->nbrs[0] = grid[i][j+1];
  }

  fi(0,nrows){
    auto line = board[i];
    int lft(0);
    while(line[lft] == ' ') lft++;


    int rt(len(line)-1);
    while(line[rt]==' ') rt--;


    if(line[lft]== '.' && line[rt] == '.'){
      grid[i][lft]->nbrs[2] = grid[i][rt];
      grid[i][rt]->nbrs[0] = grid[i][lft];
    }
  }

  fi(0,ncols){
    int top(0);
    while(board[top][i] == ' ') top++;
    int bottom(nrows-1);
    while(board[bottom][i] == ' ') bottom--;

    if(board[top][i]== '.' && board[bottom][i] == '.'){
      grid[top][i]->nbrs[3] = grid[bottom][i];
      grid[bottom][i]->nbrs[1] = grid[top][i];
    }
  }



  
  vector<pii> commands;
  int idx(0);
  int nc = len(lines[nrows]);
  for(int i=0; i<=nc && idx<nc; i++){

    if(i == nc){
      string nummy = lines[nrows].substr(idx,i-idx);
      int m = stoi(nummy);
      commands.push_back({m,0});
    }
    else{
      char ch = lines[nrows][i];
      if(ch=='R' || ch == 'L'){

        string nummy = lines[nrows].substr(idx,i-idx);

        int m = stoi(nummy);
        if(ch == 'R') commands.push_back({m,1});
        else commands.push_back({m,-1});

        idx=i+1;
      }
    }

  }

  //vecifying
  //for(auto cc : commands) cout << cc.first << "," << cc.second << endl;


  node * curr = nullptr;
  for(int c=0; c<len(grid[0]); c++){
    if(grid[0][c] != nullptr) {
      curr = grid[0][c];
      cout << "found starting point! its at column:" << c << endl;
      break;
    }

  }
  assert(curr != nullptr);

  int dir = 0;
  for(auto comm : commands){
    auto [m,d] = comm;
    curr = move(comm , curr, dir, board);

   
  }
  
  cout << "final position:" << curr->row+1 << "," << curr->col+1 << "," << endl;

  int score = 1000*(curr->row+1) + 4*(curr->col+1) + dir;
  cout << "score:" << score << endl;

}
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
 
