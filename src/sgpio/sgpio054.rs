#[doc = "Register `SGPIO054` reader"]
pub type R = crate::R<Sgpio054Spec>;
#[doc = "Register `SGPIO054` writer"]
pub type W = crate::W<Sgpio054Spec>;
#[doc = "Field `INTStatusOfSGPIO160` reader - Interrupt Status of SGPIO_160"]
pub type IntstatusOfSgpio160R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO161` reader - Interrupt Status of SGPIO_161"]
pub type IntstatusOfSgpio161R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO162` reader - Interrupt Status of SGPIO_162"]
pub type IntstatusOfSgpio162R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO163` reader - Interrupt Status of SGPIO_163"]
pub type IntstatusOfSgpio163R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO164` reader - Interrupt Status of SGPIO_164"]
pub type IntstatusOfSgpio164R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO165` reader - Interrupt Status of SGPIO_165"]
pub type IntstatusOfSgpio165R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO166` reader - Interrupt Status of SGPIO_166"]
pub type IntstatusOfSgpio166R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO167` reader - Interrupt Status of SGPIO_167"]
pub type IntstatusOfSgpio167R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO168` reader - Interrupt Status of SGPIO_168"]
pub type IntstatusOfSgpio168R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO169` reader - Interrupt Status of SGPIO_169"]
pub type IntstatusOfSgpio169R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO170` reader - Interrupt Status of SGPIO_170"]
pub type IntstatusOfSgpio170R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO171` reader - Interrupt Status of SGPIO_171"]
pub type IntstatusOfSgpio171R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO172` reader - Interrupt Status of SGPIO_172"]
pub type IntstatusOfSgpio172R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO173` reader - Interrupt Status of SGPIO_173"]
pub type IntstatusOfSgpio173R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO174` reader - Interrupt Status of SGPIO_174"]
pub type IntstatusOfSgpio174R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO175` reader - Interrupt Status of SGPIO_175"]
pub type IntstatusOfSgpio175R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO176` reader - Interrupt Status of SGPIO_176"]
pub type IntstatusOfSgpio176R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO177` reader - Interrupt Status of SGPIO_177"]
pub type IntstatusOfSgpio177R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO178` reader - Interrupt Status of SGPIO_178"]
pub type IntstatusOfSgpio178R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO179` reader - Interrupt Status of SGPIO_179"]
pub type IntstatusOfSgpio179R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO180` reader - Interrupt Status of SGPIO_180"]
pub type IntstatusOfSgpio180R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO181` reader - Interrupt Status of SGPIO_181"]
pub type IntstatusOfSgpio181R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO182` reader - Interrupt Status of SGPIO_182"]
pub type IntstatusOfSgpio182R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO183` reader - Interrupt Status of SGPIO_183"]
pub type IntstatusOfSgpio183R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO184` reader - Interrupt Status of SGPIO_184"]
pub type IntstatusOfSgpio184R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO185` reader - Interrupt Status of SGPIO_185"]
pub type IntstatusOfSgpio185R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO186` reader - Interrupt Status of SGPIO_186"]
pub type IntstatusOfSgpio186R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO187` reader - Interrupt Status of SGPIO_187"]
pub type IntstatusOfSgpio187R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO188` reader - Interrupt Status of SGPIO_188"]
pub type IntstatusOfSgpio188R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO189` reader - Interrupt Status of SGPIO_189"]
pub type IntstatusOfSgpio189R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO190` reader - Interrupt Status of SGPIO_190"]
pub type IntstatusOfSgpio190R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO191` reader - Interrupt Status of SGPIO_191"]
pub type IntstatusOfSgpio191R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_160"]
    #[inline(always)]
    pub fn intstatus_of_sgpio160(&self) -> IntstatusOfSgpio160R {
        IntstatusOfSgpio160R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_161"]
    #[inline(always)]
    pub fn intstatus_of_sgpio161(&self) -> IntstatusOfSgpio161R {
        IntstatusOfSgpio161R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_162"]
    #[inline(always)]
    pub fn intstatus_of_sgpio162(&self) -> IntstatusOfSgpio162R {
        IntstatusOfSgpio162R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_163"]
    #[inline(always)]
    pub fn intstatus_of_sgpio163(&self) -> IntstatusOfSgpio163R {
        IntstatusOfSgpio163R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_164"]
    #[inline(always)]
    pub fn intstatus_of_sgpio164(&self) -> IntstatusOfSgpio164R {
        IntstatusOfSgpio164R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_165"]
    #[inline(always)]
    pub fn intstatus_of_sgpio165(&self) -> IntstatusOfSgpio165R {
        IntstatusOfSgpio165R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_166"]
    #[inline(always)]
    pub fn intstatus_of_sgpio166(&self) -> IntstatusOfSgpio166R {
        IntstatusOfSgpio166R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_167"]
    #[inline(always)]
    pub fn intstatus_of_sgpio167(&self) -> IntstatusOfSgpio167R {
        IntstatusOfSgpio167R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_168"]
    #[inline(always)]
    pub fn intstatus_of_sgpio168(&self) -> IntstatusOfSgpio168R {
        IntstatusOfSgpio168R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_169"]
    #[inline(always)]
    pub fn intstatus_of_sgpio169(&self) -> IntstatusOfSgpio169R {
        IntstatusOfSgpio169R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_170"]
    #[inline(always)]
    pub fn intstatus_of_sgpio170(&self) -> IntstatusOfSgpio170R {
        IntstatusOfSgpio170R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_171"]
    #[inline(always)]
    pub fn intstatus_of_sgpio171(&self) -> IntstatusOfSgpio171R {
        IntstatusOfSgpio171R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_172"]
    #[inline(always)]
    pub fn intstatus_of_sgpio172(&self) -> IntstatusOfSgpio172R {
        IntstatusOfSgpio172R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_173"]
    #[inline(always)]
    pub fn intstatus_of_sgpio173(&self) -> IntstatusOfSgpio173R {
        IntstatusOfSgpio173R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_174"]
    #[inline(always)]
    pub fn intstatus_of_sgpio174(&self) -> IntstatusOfSgpio174R {
        IntstatusOfSgpio174R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_175"]
    #[inline(always)]
    pub fn intstatus_of_sgpio175(&self) -> IntstatusOfSgpio175R {
        IntstatusOfSgpio175R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_176"]
    #[inline(always)]
    pub fn intstatus_of_sgpio176(&self) -> IntstatusOfSgpio176R {
        IntstatusOfSgpio176R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_177"]
    #[inline(always)]
    pub fn intstatus_of_sgpio177(&self) -> IntstatusOfSgpio177R {
        IntstatusOfSgpio177R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_178"]
    #[inline(always)]
    pub fn intstatus_of_sgpio178(&self) -> IntstatusOfSgpio178R {
        IntstatusOfSgpio178R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_179"]
    #[inline(always)]
    pub fn intstatus_of_sgpio179(&self) -> IntstatusOfSgpio179R {
        IntstatusOfSgpio179R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_180"]
    #[inline(always)]
    pub fn intstatus_of_sgpio180(&self) -> IntstatusOfSgpio180R {
        IntstatusOfSgpio180R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_181"]
    #[inline(always)]
    pub fn intstatus_of_sgpio181(&self) -> IntstatusOfSgpio181R {
        IntstatusOfSgpio181R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_182"]
    #[inline(always)]
    pub fn intstatus_of_sgpio182(&self) -> IntstatusOfSgpio182R {
        IntstatusOfSgpio182R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_183"]
    #[inline(always)]
    pub fn intstatus_of_sgpio183(&self) -> IntstatusOfSgpio183R {
        IntstatusOfSgpio183R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_184"]
    #[inline(always)]
    pub fn intstatus_of_sgpio184(&self) -> IntstatusOfSgpio184R {
        IntstatusOfSgpio184R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_185"]
    #[inline(always)]
    pub fn intstatus_of_sgpio185(&self) -> IntstatusOfSgpio185R {
        IntstatusOfSgpio185R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_186"]
    #[inline(always)]
    pub fn intstatus_of_sgpio186(&self) -> IntstatusOfSgpio186R {
        IntstatusOfSgpio186R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_187"]
    #[inline(always)]
    pub fn intstatus_of_sgpio187(&self) -> IntstatusOfSgpio187R {
        IntstatusOfSgpio187R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_188"]
    #[inline(always)]
    pub fn intstatus_of_sgpio188(&self) -> IntstatusOfSgpio188R {
        IntstatusOfSgpio188R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_189"]
    #[inline(always)]
    pub fn intstatus_of_sgpio189(&self) -> IntstatusOfSgpio189R {
        IntstatusOfSgpio189R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_190"]
    #[inline(always)]
    pub fn intstatus_of_sgpio190(&self) -> IntstatusOfSgpio190R {
        IntstatusOfSgpio190R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_191"]
    #[inline(always)]
    pub fn intstatus_of_sgpio191(&self) -> IntstatusOfSgpio191R {
        IntstatusOfSgpio191R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio054Spec;
impl crate::RegisterSpec for Sgpio054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio054::R`](R) reader structure"]
impl crate::Readable for Sgpio054Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio054::W`](W) writer structure"]
impl crate::Writable for Sgpio054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO054 to value 0"]
impl crate::Resettable for Sgpio054Spec {}
