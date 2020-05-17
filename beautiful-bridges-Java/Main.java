import java.util.ArrayList;
import java.util.Scanner;

public class Main {

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
            int[][] matriz = new int[n + 1][n + 1];
            matriz[0][0]=0;
            matriz[0][1]=0;
            for (int i = 1; i <= n; i++) {
                input = registro.nextLine();
                lista1 = input.split(" ");
                matriz[i][0] = Integer.parseInt(lista1[0]);
                matriz[i][1] = Integer.parseInt(lista1[1]);
                if(!(matriz[i][0]>=0 && matriz[i][0]<=Math.pow(10,5) && matriz[i][1]>=0 &&matriz[i][1]<h && matriz[i-1][0]<matriz[i][0]) && i>1){
                    verificar=false;
                }

            }
            if(verificar){
                System.out.println(cost(n, h, a, b, matriz));
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
                distancia=matrix[i][0]-matrix[j][0];
                altura=h-matrix[j][1];
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
        int i=n-1;
        int k=n-1;
        ArrayList <Integer> lista=new ArrayList<Integer>();
        lista.add(i);
        while (i>0 && k>0){
            if(costo[i][k]!=costo[i-1][k]){
                lista.add(i);
                i=i-1;
                k=k-1;
            }else{
                i=i-1;
            }
        }
        ArrayList<Integer> distancias=new ArrayList<Integer>();
        for (int v=0;v<lista.size()-1;v++){
            distancias.add(matrix[lista.get(v)][0]-matrix[lista.get(v+1)][0]);
        }
        /*for (int z:distancias){
            System.out.println(z);
        }
        /*System.out.println("Pilares");
        for (int z:lista){
            System.out.println(z);
        }*/

        /*for(int i = 0; i < costo.length; i++){
            for(int j = 0; j < costo[i].length; j++){
                System.out.print(costo[i][j] + " ");	// Imprime elemento
            }
            System.out.println();	// Imprime salto de línea
        }*/
        return costo[costo.length-1][costo[0].length-1];
    }
    public static long costo1(long h,long d,long a,long b){
        long m=(a*h)+((d*d)*b);
        return m;
    }
    public static boolean radio(int x,int y,int heigh,double rad){
        double r=Math.sqrt((Math.pow(rad,2))+Math.pow(x,2));
        return (heigh-r)>y;
    }
}
