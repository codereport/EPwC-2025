unit U_BinaryCounterReduction;

interface

procedure Min_12_main;


implementation
uses  Spring, Spring.Collections, System.Math, System.Generics.Collections, System.Sysutils;

const crlf = #13#10;
var   data, counter : IList<integer>;
type  TOp  = TFunc<integer,integer,integer>;

function least(x,y:integer):integer;   begin  result := ifthen(x<y, x, y);  end;
function most (x,y:integer):integer;   begin  result := ifthen(x>y, x, y);  end;


procedure Add(carry:integer; const Cmp:TOp);
begin
      for var i := 1 to counter.Count do begin
           if counter[i] = 0 then begin
                                     counter[i] := carry;
                                     carry   := 0;
                                     break;
                                  end;
           carry := Cmp(counter[i], carry);
           counter[i] := 0;
      end;
      if carry <> 0 then counter.Add(carry);
end;


function reduce(const Cmp:TOp):integer;
begin
      if counter.Count = 0 then exit(0);

      result := counter[1];
      for var c in counter do
           if c<>0 then result := Cmp(c, result);
end;


const data0 : TArray<integer> = [9, 13, 7, 124, 32, 17, 8, 32, 237, 417, 41, 42, 13, 14, 15];

procedure Min_12_main;
begin
      // Spring4D library
      for var x in TEnumerable.From(data0).Ordered.Take(2)              do writeln('Min : ',x);
      for var x in TEnumerable.From(data0).Ordered.TakeLast(2).Reversed do writeln('Max : ',x);   writeln;

      counter := TCollections.CreateList<integer>;
      data    := TCollections.CreateList<integer>(data0);

      // by hand
      for var d in data do add(d,least);
      var x := reduce(least);
      writeln('Min : ', x); 

      counter.Clear;

      for var d in data do add(d,most);
      var x := reduce(most);	  
      writeln('Max : ', x); 
end;

end.



