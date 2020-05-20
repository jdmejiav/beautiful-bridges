import java.util.ArrayList;
import java.util.Scanner;

public class Main2 {

    public static void main(String[] args) {
        Scanner registro=new Scanner(System.in);
        String input=registro.nextLine();
        String lista1[]=input.split(" ");
        int n=Integer.parseInt(lista1[0]);
        long h=Integer.parseInt(lista1[1]);
        long a=Integer.parseInt(lista1[2]);
        long b=Integer.parseInt(lista1[3]);
        boolean verificar=true;
        if (n>=2 && n<=Math.pow(10,4) && h>=1 && h<=Math.pow(10,5)&&a>=1&&a<=Math.pow(10,4)&&b>=1&&b<=Math.pow(10,4)) {
            int[][] matriz = new int[n+1][2];
            for (int i = 1; i <= n; i++) {
                input = registro.nextLine();
                lista1 = input.split(" ");
                matriz[i][0] = Integer.parseInt(lista1[0]);
                matriz[i][1] = Integer.parseInt(lista1[1]);
                if(i>1) {
                    if (!(matriz[i][0] >= 0 && matriz[i][0] <= Math.pow(10, 5) && matriz[i][1] >= 0 && matriz[i][1] < h && matriz[i - 1][0] < matriz[i][0])) {
                        verificar = false;
                    }
                }

            }
    /*
            System.out.println("\nposiciones");
            for(int A = 0; A < matriz.length; A++){
                for(int j = 0; j < matriz[A].length; j++){
                    System.out.print(matriz[A][j] + " ");	// Imprime elemento
                }
                System.out.println();	// Imprime salto de línea
            }//*/


            if(verificar){
                long result=cost(n, h, a, b, matriz);
                if (result!= -1){
                    System.out.println(result);
                }else{
                    System.out.println("impossible");
                }
            }else{
                System.out.println("impossible");
            }

        }else{
            System.out.println("impossible");
        }
    }
    public static long cost(int n,long h,long a,long b,int [][] matrix){
        n=n+1;
        long [] [] costo=new long [n] [n];
        for (int i=0;i<n;i++){
            costo[0][i]=0;
        }
        for(int i=0;i<n;i++){
            costo[0][i]=0;
        }
        long altura;
        long distancia;
        for(int i=1;i<n;i++){
            for(int j=1;j<n;j++){
                //System.out.println("I= "+i+"\nJ="+j);
                distancia=Math.abs(matrix[i][0]-matrix[j][0]);
               // System.out.println("distancia:\nmatrix[i][0]-matrix[j][0]= "+matrix[i][0]+"-"+matrix[j][0]+"="+distancia);
                altura=h-matrix[j][1];
                //System.out.println("altura=h-matrix[j][1]= "+h+"-"+matrix[j][1]+"="+altura);
                if(i==1){
                    costo[i][j]=costo1(altura,distancia,a,b)+costo[i][i];
                }else{
                    if(costo1(altura,distancia,a,b)+costo[i-1][i]<costo[i-1][j]){
                        costo[i][j]=costo1(altura,distancia,a,b)+costo[i-1][i];
                    }else{
                        costo[i][j]=costo[i-1][j];
                    }
                }

            }
        }


        int i=n-2;
        int k=n-1;
        ArrayList <Integer> lista=new ArrayList<Integer>();
        while (i>0 && k>0){
            if(costo[i][k]!=costo[i-1][k]&&i!=1){
                lista.add(i);
                i=i-1;
                k=k-1;
            }else{
                i=i-1;
            }
        }
        lista.add(1);
        lista.add(n-1);

        long pilares [] [] = new long [3][n];
        for (int j = 0; j <3 ; j++) {
            pilares[j][0]=0;
        }
        for (int j = 1; j <n ; j++) {
            if (lista.contains(j)){
                pilares[0][j]=1;
            }else{
                pilares[0][j]=0;
            }
        }
        for (int j = 1; j <n-1; j++) {
            if(pilares[0][j]==1 && pilares[0][j+1]==1){
                pilares[1][j]=matrix[j+1][0]-matrix[j][0];
            }else{
                if(pilares[0][j]==1 && pilares[0][j-1]==1){
                    pilares[1][j]=pilares[1][j-1];
                }else{
                    if(pilares[0][j]==0 && pilares[0][j+1]==1){
                        pilares[1][j]=matrix[j+1][0]-matrix[j-1][0];
                    }

                }
            }
        }//*/
        pilares[1][n-1]=pilares[1][n-2];
        boolean puente=true;
        int count=0;
        for (int j = 1; j <n ; j++) {
            //System.out.println("RADIO="+pilares[1][j]/2+"\nx="+matrix[j][0]+"\ny="+matrix[j][1]);
            if(count%2==0||j>2){
                if ((radio1(matrix[j][0],matrix[j][1],h,pilares[1][j]/2))){
                    puente=false;
                    pilares[2][j]=0;
                    break;
                }else{
                    pilares[2][j]=1;
                }
            }else{
                if ((radio(matrix[j][0],matrix[j][1],h,pilares[1][j]/2))){
                    puente=false;
                    pilares[2][j]=0;
                    break;
                }else{
                    pilares[2][j]=1;
                }
            }

            count++;
        }
        /*
        System.out.println("Matriz pilares");
        for(int x = 0; x < pilares.length; x++){
            for(int j = 0; j < pilares[x].length; j++){
                System.out.print(pilares[x][j] + "\t");	// Imprime elemento
            }
            System.out.println();	// Imprime salto de línea
        }//*/
/*
        System.out.println("\nPilares");
        for (int z:lista){
            System.out.print(z+" ,");
        }//*/
/*
        System.out.println("\nCOSTO");
        for(int A = 0; A < costo.length; A++){
            for(int j = 0; j < costo[A].length; j++){
                System.out.print(costo[A][j] + " ");	// Imprime elemento
            }
            System.out.println();	// Imprime salto de línea
        }//*/
        if (puente){
            return costo[costo.length-1][costo[0].length-1];
        }
        return -1;
    }
    public static long costo1(long h,long d,long a,long b){
        long m=(a*h)+((d*d)*b);
        return m;
    }
    public static boolean radio(int x,int hy,long heigh,double rad){
        double y=Math.sqrt(Math.abs(Math.pow(rad,2)-Math.pow((x-rad),2)))+(heigh-rad);
        double r1=heigh-rad;
        double r2=(x-rad);
        return y>=heigh;
    }
    public static boolean radio1(int x,int hy,long heigh,double rad){
        double y=Math.sqrt(Math.pow(rad,2)-Math.pow((x+rad),2))+(heigh-rad);
        double r1=heigh-rad;
        double r2=(x+rad);
        return y>=heigh;
    }
}
