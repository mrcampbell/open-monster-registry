# Monster Data Structure

I propose that we take on this data structure (in pseudocode) for storing a Monster's data, in protobuf format

In this format, Nominal Data's ID will be incrementing integers, and everything else will have a generated ID.

```protobuf
message Generation {
  ID int32 = 1; // incrementing integer
  Name string = 2; // Alpha, Beta, Gamma, etc
}

message Element {
  ID int32 = 1; // incrementing integer
  Name string = 2; // Fire/Flame/Hydro/Water, whatever
}

message Stat {
  ID
}

message Species {
  ID string = 1; // either CUID or UUID, etc
  Name string = 2;
  repeating Elements Element = 3; // allows for 1..n elements

}

```