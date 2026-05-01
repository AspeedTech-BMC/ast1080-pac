#[doc = "Register `SGPIO048` reader"]
pub type R = crate::R<Sgpio048Spec>;
#[doc = "Register `SGPIO048` writer"]
pub type W = crate::W<Sgpio048Spec>;
#[doc = "Field `INTStatusOfSGPIO64` reader - Interrupt Status of SGPIO_64"]
pub type IntstatusOfSgpio64R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO65` reader - Interrupt Status of SGPIO_65"]
pub type IntstatusOfSgpio65R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO66` reader - Interrupt Status of SGPIO_66"]
pub type IntstatusOfSgpio66R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO67` reader - Interrupt Status of SGPIO_67"]
pub type IntstatusOfSgpio67R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO68` reader - Interrupt Status of SGPIO_68"]
pub type IntstatusOfSgpio68R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO69` reader - Interrupt Status of SGPIO_69"]
pub type IntstatusOfSgpio69R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO70` reader - Interrupt Status of SGPIO_70"]
pub type IntstatusOfSgpio70R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO71` reader - Interrupt Status of SGPIO_71"]
pub type IntstatusOfSgpio71R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO72` reader - Interrupt Status of SGPIO_72"]
pub type IntstatusOfSgpio72R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO73` reader - Interrupt Status of SGPIO_73"]
pub type IntstatusOfSgpio73R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO74` reader - Interrupt Status of SGPIO_74"]
pub type IntstatusOfSgpio74R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO75` reader - Interrupt Status of SGPIO_75"]
pub type IntstatusOfSgpio75R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO76` reader - Interrupt Status of SGPIO_76"]
pub type IntstatusOfSgpio76R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO77` reader - Interrupt Status of SGPIO_77"]
pub type IntstatusOfSgpio77R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO78` reader - Interrupt Status of SGPIO_78"]
pub type IntstatusOfSgpio78R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO79` reader - Interrupt Status of SGPIO_79"]
pub type IntstatusOfSgpio79R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO80` reader - Interrupt Status of SGPIO_80"]
pub type IntstatusOfSgpio80R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO81` reader - Interrupt Status of SGPIO_81"]
pub type IntstatusOfSgpio81R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO82` reader - Interrupt Status of SGPIO_82"]
pub type IntstatusOfSgpio82R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO83` reader - Interrupt Status of SGPIO_83"]
pub type IntstatusOfSgpio83R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO84` reader - Interrupt Status of SGPIO_84"]
pub type IntstatusOfSgpio84R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO85` reader - Interrupt Status of SGPIO_85"]
pub type IntstatusOfSgpio85R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO86` reader - Interrupt Status of SGPIO_86"]
pub type IntstatusOfSgpio86R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO87` reader - Interrupt Status of SGPIO_87"]
pub type IntstatusOfSgpio87R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO88` reader - Interrupt Status of SGPIO_88"]
pub type IntstatusOfSgpio88R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO89` reader - Interrupt Status of SGPIO_89"]
pub type IntstatusOfSgpio89R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO90` reader - Interrupt Status of SGPIO_90"]
pub type IntstatusOfSgpio90R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO91` reader - Interrupt Status of SGPIO_91"]
pub type IntstatusOfSgpio91R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO92` reader - Interrupt Status of SGPIO_92"]
pub type IntstatusOfSgpio92R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO93` reader - Interrupt Status of SGPIO_93"]
pub type IntstatusOfSgpio93R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO94` reader - Interrupt Status of SGPIO_94"]
pub type IntstatusOfSgpio94R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO95` reader - Interrupt Status of SGPIO_95"]
pub type IntstatusOfSgpio95R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_64"]
    #[inline(always)]
    pub fn intstatus_of_sgpio64(&self) -> IntstatusOfSgpio64R {
        IntstatusOfSgpio64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_65"]
    #[inline(always)]
    pub fn intstatus_of_sgpio65(&self) -> IntstatusOfSgpio65R {
        IntstatusOfSgpio65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_66"]
    #[inline(always)]
    pub fn intstatus_of_sgpio66(&self) -> IntstatusOfSgpio66R {
        IntstatusOfSgpio66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_67"]
    #[inline(always)]
    pub fn intstatus_of_sgpio67(&self) -> IntstatusOfSgpio67R {
        IntstatusOfSgpio67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_68"]
    #[inline(always)]
    pub fn intstatus_of_sgpio68(&self) -> IntstatusOfSgpio68R {
        IntstatusOfSgpio68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_69"]
    #[inline(always)]
    pub fn intstatus_of_sgpio69(&self) -> IntstatusOfSgpio69R {
        IntstatusOfSgpio69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_70"]
    #[inline(always)]
    pub fn intstatus_of_sgpio70(&self) -> IntstatusOfSgpio70R {
        IntstatusOfSgpio70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_71"]
    #[inline(always)]
    pub fn intstatus_of_sgpio71(&self) -> IntstatusOfSgpio71R {
        IntstatusOfSgpio71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_72"]
    #[inline(always)]
    pub fn intstatus_of_sgpio72(&self) -> IntstatusOfSgpio72R {
        IntstatusOfSgpio72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_73"]
    #[inline(always)]
    pub fn intstatus_of_sgpio73(&self) -> IntstatusOfSgpio73R {
        IntstatusOfSgpio73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_74"]
    #[inline(always)]
    pub fn intstatus_of_sgpio74(&self) -> IntstatusOfSgpio74R {
        IntstatusOfSgpio74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_75"]
    #[inline(always)]
    pub fn intstatus_of_sgpio75(&self) -> IntstatusOfSgpio75R {
        IntstatusOfSgpio75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_76"]
    #[inline(always)]
    pub fn intstatus_of_sgpio76(&self) -> IntstatusOfSgpio76R {
        IntstatusOfSgpio76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_77"]
    #[inline(always)]
    pub fn intstatus_of_sgpio77(&self) -> IntstatusOfSgpio77R {
        IntstatusOfSgpio77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_78"]
    #[inline(always)]
    pub fn intstatus_of_sgpio78(&self) -> IntstatusOfSgpio78R {
        IntstatusOfSgpio78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_79"]
    #[inline(always)]
    pub fn intstatus_of_sgpio79(&self) -> IntstatusOfSgpio79R {
        IntstatusOfSgpio79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_80"]
    #[inline(always)]
    pub fn intstatus_of_sgpio80(&self) -> IntstatusOfSgpio80R {
        IntstatusOfSgpio80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_81"]
    #[inline(always)]
    pub fn intstatus_of_sgpio81(&self) -> IntstatusOfSgpio81R {
        IntstatusOfSgpio81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_82"]
    #[inline(always)]
    pub fn intstatus_of_sgpio82(&self) -> IntstatusOfSgpio82R {
        IntstatusOfSgpio82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_83"]
    #[inline(always)]
    pub fn intstatus_of_sgpio83(&self) -> IntstatusOfSgpio83R {
        IntstatusOfSgpio83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_84"]
    #[inline(always)]
    pub fn intstatus_of_sgpio84(&self) -> IntstatusOfSgpio84R {
        IntstatusOfSgpio84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_85"]
    #[inline(always)]
    pub fn intstatus_of_sgpio85(&self) -> IntstatusOfSgpio85R {
        IntstatusOfSgpio85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_86"]
    #[inline(always)]
    pub fn intstatus_of_sgpio86(&self) -> IntstatusOfSgpio86R {
        IntstatusOfSgpio86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_87"]
    #[inline(always)]
    pub fn intstatus_of_sgpio87(&self) -> IntstatusOfSgpio87R {
        IntstatusOfSgpio87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_88"]
    #[inline(always)]
    pub fn intstatus_of_sgpio88(&self) -> IntstatusOfSgpio88R {
        IntstatusOfSgpio88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_89"]
    #[inline(always)]
    pub fn intstatus_of_sgpio89(&self) -> IntstatusOfSgpio89R {
        IntstatusOfSgpio89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_90"]
    #[inline(always)]
    pub fn intstatus_of_sgpio90(&self) -> IntstatusOfSgpio90R {
        IntstatusOfSgpio90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_91"]
    #[inline(always)]
    pub fn intstatus_of_sgpio91(&self) -> IntstatusOfSgpio91R {
        IntstatusOfSgpio91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_92"]
    #[inline(always)]
    pub fn intstatus_of_sgpio92(&self) -> IntstatusOfSgpio92R {
        IntstatusOfSgpio92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_93"]
    #[inline(always)]
    pub fn intstatus_of_sgpio93(&self) -> IntstatusOfSgpio93R {
        IntstatusOfSgpio93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_94"]
    #[inline(always)]
    pub fn intstatus_of_sgpio94(&self) -> IntstatusOfSgpio94R {
        IntstatusOfSgpio94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_95"]
    #[inline(always)]
    pub fn intstatus_of_sgpio95(&self) -> IntstatusOfSgpio95R {
        IntstatusOfSgpio95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio048Spec;
impl crate::RegisterSpec for Sgpio048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio048::R`](R) reader structure"]
impl crate::Readable for Sgpio048Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio048::W`](W) writer structure"]
impl crate::Writable for Sgpio048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO048 to value 0"]
impl crate::Resettable for Sgpio048Spec {}
