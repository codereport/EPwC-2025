{$APPTYPE CONSOLE}
program Project_epwc;
uses
  System.SysUtils,
  U_unique_count           in 'unique_count\U_unique_count.pas',
  U_MinMax_Element         in 'minmax_elem\U_MinMax_Element.pas',
  U_Gen_Data               in 'U_Gen_Data.pas',
  U_Test_Instrumented      in 'instrumented\U_Test_Instrumented.pas',
  U_Instrumented           in 'instrumented\U_Instrumented.pas',
  U_BinaryCounterReduction in 'binary_counter\U_BinaryCounterReduction.pas';

begin
   // Unique_Count_Main;
   // MinMax_Main;
   // Instrumented_Main;
   Min_12_main;

   readln;
end.






