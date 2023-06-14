– Identify you and your programming partner by name
Noah Santagata and Isabella Martinez

– Acknowledge help you may have received from or collaborative work
you may have undertaken with others
We have gone to Isaacs's and well as Daniel's TA hours during the development of our code. Isaac helped us with iterating through our RGB file and trimming it. We have also talked with peers Thomas Fargnoli and Michaela Healy who gave us examples of how we would use the functions declared in bitpack. From their conceptual help, we were able to finish those function declarations, with Array2 taken from code provided by Prof. Daniels. When we went to Daniel's office hours, he helped us troubleshoot how we were placing our ypbpr values when converting back to RGB in decompression. Additionally, we both have contributed to this code in the presence of one another whether in person or over discord. For online resources, we heavily used the documentation for csc411_repgio to understand how we are using this to both read in for decompression and write out for compression. 

– Identify what has been correctly implemented and what has not
What has been correctly implemented:
Compression 
- reading in RGB image
- trimming col or row of RGB image if necessary(odd)
- changing RGB numbers to floats using the denominator
- transforming RGB into Component Video Color 
- calculating avg_pb
- calculating avg-pr
- calculating a,b,c,d using DCT
- converting b,c,d into five-bit signed values
- packing values above into 32-bit word
- creating a compressed file
- bitpack is correctly implemented
Decompression
- unpacking word into a,b,c,d,pb,pr values

What has not been correctly implemented:
We believe we have successfully implemented everything

– Explain the architecture of your solution
The architecture of our solution is as follows: we read in the ppm file, receiving its height, width, and denominator. Making sure the grid is even across, we also subtract the size of height and width by 1 if necessary.

Afterwards, using the conversion formula found on page 10, we were able to retrieve our Y, Pb, and Pr values by using elements of our RGB that we received and placed into a vector. After the conversion, each element of a vector will have a newly updated value in the form of its y, Pb, and Pr, {Y,Pb,Pr}. Operating in a manner where only 2x2 grid sections are focused on, we take the combined values of Pb and Pr separately, and divide them by 4 to get their new value, after multiplying them by 50 first. Using the given function, “index_of_chroma”, we are able to convert Pb and Pr into a working number we will use.

Afterwards, we run our numbers through a Direct Cosine Transformation, (DCT for short) where values “a, b, c and d” are given 4 “Y” values, based on the 2x2 grid’s first element of each cell. With a, b, c, and d discovered, we are able to use them in a folder called “bitpack.rs” where special operations are performed on its given values, based on the set conditions required for its binary value.

With the binary value of our word given, we print out the results, thus giving the newly compressed image.

There’s also the functionality of decompression. This function packs our values such as “a,b,c,d,pb, pr”, and using operations within bitpack.rs like “gets” and “getu”, we were able to return their numbers into decimal digits. From here, we used an inverse condition based on the DCT where we multiplied by 4 instead of dividing by it. Then, using the other given function “chroma_of_index” do we turn Pb and Pr into specialized numbers that are used within another given formula where we find the individual red, green, and blue values. Dividing their values by 50 and multiplying by 4, we multiply each number by an Inverse DCT, and apply it to a vector accordingly. This decompressess our image, giving it back just as it was originally found.


– Say approximately how many hours you have spent analyzing the
problems posed in the assignment

We have approximately spent about 10-15 hours fully analyzing and understanding each problem/section within this assignment.

– Say approximately how many hours you have spent solving the problems after your analysis

When it came to solving problems, we have approximately spent about 100 hours.
