
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


using vi=vector<int>;
using si=set<int>;
using vd=vector<double>;
using graph = map<int,multiset<int>>;

using ull=long long;
using pii = pair<ull,ull>;
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
  size_t ncols = len(lines[0]);
  for(auto line : lines) ncols = max(ncols,len(line));
  fi(0,nrows) board.push_back(lines[i]);
  
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
  cout << "nrows, ncols:" << nrows << "," << ncols << endl;

  tri start = {0,50,0};

  vector<pii> dirvecs = {{0,1},{1,0},{0,-1},{-1,0}};

  auto normalize = [&](int & r, int & c) {

                     if(r<0) r=nrows-1;
                     else if(r>=nrows) r = 0;
                     int nc = len(board[r]);
                     if(c<0) c=nc-1;
                     if(c>=nc) c = 0;
                   };

  auto next = [&](tri &pos, int move, int rot)
              {
                printf(" command: move %d and rot %d....",move,rot);
                auto dir = dirvecs[get<2>(pos)];

                while(move>0){
                  int r = get<0>(pos);
                  int c = get<1>(pos);
                  printf("starting (r:%d, c:%d,dir:%d),",r,c,get<2>(pos));
                  
                  r +=dir.first;
                  c +=dir.second;


                  normalize(r,c);
                  
                 
                   
                  while(board[r][c]==' '){
                    r+=dir.first; c+=dir.second;
                    normalize(r,c);
                  }

                  
                  if(board[r][c]=='#'){
                    move = 0;
                  }
                  else if(board[r][c]=='.'){
                    move--;
                    get<0>(pos)=r;
                    get<1>(pos)=c;

                  }
                  else{
                    cout << "u forgot something: r,c:" << r << "," << c << ">" << board[r][c] << "<" << endl;
                    assert(false);
                  }
                  
                 
                };
                 int r = get<0>(pos);
                  int c = get<1>(pos);
                  printf(" \nmoved to: r:%d, c:%d\n",r,c);

                
                  int nurot = (get<2>(pos) + rot) %4  ;
                 if(nurot<0) nurot += 4;

                 get<2>(pos) = nurot;

              };

  for(auto comm : commands){
    next(start,comm.first, comm.second);
  }
  
  cout << "final position:" << get<0>(start)+1 << "," << get<1>(start)+1 << "," << get<2>(start) << endl;

  int score = 1000*(get<0>(start)+1) + 4*(get<1>(start)+1) + get<2>(start);
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
 
