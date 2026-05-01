#[doc = "Register `SGPIO` reader"]
pub type R = crate::R<SgpioSpec>;
#[doc = "Register `SGPIO` writer"]
pub type W = crate::W<SgpioSpec>;
#[doc = "Field `INTStatusOfSGPIO192` reader - Interrupt Status of SGPIO_192"]
pub type IntstatusOfSgpio192R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO193` reader - Interrupt Status of SGPIO_193"]
pub type IntstatusOfSgpio193R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO194` reader - Interrupt Status of SGPIO_194"]
pub type IntstatusOfSgpio194R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO195` reader - Interrupt Status of SGPIO_195"]
pub type IntstatusOfSgpio195R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO196` reader - Interrupt Status of SGPIO_196"]
pub type IntstatusOfSgpio196R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO197` reader - Interrupt Status of SGPIO_197"]
pub type IntstatusOfSgpio197R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO198` reader - Interrupt Status of SGPIO_198"]
pub type IntstatusOfSgpio198R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO199` reader - Interrupt Status of SGPIO_199"]
pub type IntstatusOfSgpio199R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO200` reader - Interrupt Status of SGPIO_200"]
pub type IntstatusOfSgpio200R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO201` reader - Interrupt Status of SGPIO_201"]
pub type IntstatusOfSgpio201R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO202` reader - Interrupt Status of SGPIO_202"]
pub type IntstatusOfSgpio202R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO203` reader - Interrupt Status of SGPIO_203"]
pub type IntstatusOfSgpio203R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO204` reader - Interrupt Status of SGPIO_204"]
pub type IntstatusOfSgpio204R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO205` reader - Interrupt Status of SGPIO_205"]
pub type IntstatusOfSgpio205R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO206` reader - Interrupt Status of SGPIO_206"]
pub type IntstatusOfSgpio206R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO207` reader - Interrupt Status of SGPIO_207"]
pub type IntstatusOfSgpio207R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO208` reader - Interrupt Status of SGPIO_208"]
pub type IntstatusOfSgpio208R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO209` reader - Interrupt Status of SGPIO_209"]
pub type IntstatusOfSgpio209R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO210` reader - Interrupt Status of SGPIO_210"]
pub type IntstatusOfSgpio210R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO211` reader - Interrupt Status of SGPIO_211"]
pub type IntstatusOfSgpio211R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO212` reader - Interrupt Status of SGPIO_212"]
pub type IntstatusOfSgpio212R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO213` reader - Interrupt Status of SGPIO_213"]
pub type IntstatusOfSgpio213R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO214` reader - Interrupt Status of SGPIO_214"]
pub type IntstatusOfSgpio214R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO215` reader - Interrupt Status of SGPIO_215"]
pub type IntstatusOfSgpio215R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO216` reader - Interrupt Status of SGPIO_216"]
pub type IntstatusOfSgpio216R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO217` reader - Interrupt Status of SGPIO_217"]
pub type IntstatusOfSgpio217R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO218` reader - Interrupt Status of SGPIO_218"]
pub type IntstatusOfSgpio218R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO219` reader - Interrupt Status of SGPIO_219"]
pub type IntstatusOfSgpio219R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO220` reader - Interrupt Status of SGPIO_220"]
pub type IntstatusOfSgpio220R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO221` reader - Interrupt Status of SGPIO_221"]
pub type IntstatusOfSgpio221R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO222` reader - Interrupt Status of SGPIO_222"]
pub type IntstatusOfSgpio222R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO223` reader - Interrupt Status of SGPIO_223"]
pub type IntstatusOfSgpio223R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_192"]
    #[inline(always)]
    pub fn intstatus_of_sgpio192(&self) -> IntstatusOfSgpio192R {
        IntstatusOfSgpio192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_193"]
    #[inline(always)]
    pub fn intstatus_of_sgpio193(&self) -> IntstatusOfSgpio193R {
        IntstatusOfSgpio193R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_194"]
    #[inline(always)]
    pub fn intstatus_of_sgpio194(&self) -> IntstatusOfSgpio194R {
        IntstatusOfSgpio194R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_195"]
    #[inline(always)]
    pub fn intstatus_of_sgpio195(&self) -> IntstatusOfSgpio195R {
        IntstatusOfSgpio195R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_196"]
    #[inline(always)]
    pub fn intstatus_of_sgpio196(&self) -> IntstatusOfSgpio196R {
        IntstatusOfSgpio196R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_197"]
    #[inline(always)]
    pub fn intstatus_of_sgpio197(&self) -> IntstatusOfSgpio197R {
        IntstatusOfSgpio197R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_198"]
    #[inline(always)]
    pub fn intstatus_of_sgpio198(&self) -> IntstatusOfSgpio198R {
        IntstatusOfSgpio198R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_199"]
    #[inline(always)]
    pub fn intstatus_of_sgpio199(&self) -> IntstatusOfSgpio199R {
        IntstatusOfSgpio199R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_200"]
    #[inline(always)]
    pub fn intstatus_of_sgpio200(&self) -> IntstatusOfSgpio200R {
        IntstatusOfSgpio200R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_201"]
    #[inline(always)]
    pub fn intstatus_of_sgpio201(&self) -> IntstatusOfSgpio201R {
        IntstatusOfSgpio201R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_202"]
    #[inline(always)]
    pub fn intstatus_of_sgpio202(&self) -> IntstatusOfSgpio202R {
        IntstatusOfSgpio202R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_203"]
    #[inline(always)]
    pub fn intstatus_of_sgpio203(&self) -> IntstatusOfSgpio203R {
        IntstatusOfSgpio203R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_204"]
    #[inline(always)]
    pub fn intstatus_of_sgpio204(&self) -> IntstatusOfSgpio204R {
        IntstatusOfSgpio204R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_205"]
    #[inline(always)]
    pub fn intstatus_of_sgpio205(&self) -> IntstatusOfSgpio205R {
        IntstatusOfSgpio205R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_206"]
    #[inline(always)]
    pub fn intstatus_of_sgpio206(&self) -> IntstatusOfSgpio206R {
        IntstatusOfSgpio206R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_207"]
    #[inline(always)]
    pub fn intstatus_of_sgpio207(&self) -> IntstatusOfSgpio207R {
        IntstatusOfSgpio207R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_208"]
    #[inline(always)]
    pub fn intstatus_of_sgpio208(&self) -> IntstatusOfSgpio208R {
        IntstatusOfSgpio208R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_209"]
    #[inline(always)]
    pub fn intstatus_of_sgpio209(&self) -> IntstatusOfSgpio209R {
        IntstatusOfSgpio209R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_210"]
    #[inline(always)]
    pub fn intstatus_of_sgpio210(&self) -> IntstatusOfSgpio210R {
        IntstatusOfSgpio210R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_211"]
    #[inline(always)]
    pub fn intstatus_of_sgpio211(&self) -> IntstatusOfSgpio211R {
        IntstatusOfSgpio211R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_212"]
    #[inline(always)]
    pub fn intstatus_of_sgpio212(&self) -> IntstatusOfSgpio212R {
        IntstatusOfSgpio212R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_213"]
    #[inline(always)]
    pub fn intstatus_of_sgpio213(&self) -> IntstatusOfSgpio213R {
        IntstatusOfSgpio213R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_214"]
    #[inline(always)]
    pub fn intstatus_of_sgpio214(&self) -> IntstatusOfSgpio214R {
        IntstatusOfSgpio214R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_215"]
    #[inline(always)]
    pub fn intstatus_of_sgpio215(&self) -> IntstatusOfSgpio215R {
        IntstatusOfSgpio215R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_216"]
    #[inline(always)]
    pub fn intstatus_of_sgpio216(&self) -> IntstatusOfSgpio216R {
        IntstatusOfSgpio216R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_217"]
    #[inline(always)]
    pub fn intstatus_of_sgpio217(&self) -> IntstatusOfSgpio217R {
        IntstatusOfSgpio217R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_218"]
    #[inline(always)]
    pub fn intstatus_of_sgpio218(&self) -> IntstatusOfSgpio218R {
        IntstatusOfSgpio218R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_219"]
    #[inline(always)]
    pub fn intstatus_of_sgpio219(&self) -> IntstatusOfSgpio219R {
        IntstatusOfSgpio219R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_220"]
    #[inline(always)]
    pub fn intstatus_of_sgpio220(&self) -> IntstatusOfSgpio220R {
        IntstatusOfSgpio220R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_221"]
    #[inline(always)]
    pub fn intstatus_of_sgpio221(&self) -> IntstatusOfSgpio221R {
        IntstatusOfSgpio221R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_222"]
    #[inline(always)]
    pub fn intstatus_of_sgpio222(&self) -> IntstatusOfSgpio222R {
        IntstatusOfSgpio222R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_223"]
    #[inline(always)]
    pub fn intstatus_of_sgpio223(&self) -> IntstatusOfSgpio223R {
        IntstatusOfSgpio223R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgpioSpec;
impl crate::RegisterSpec for SgpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio::R`](R) reader structure"]
impl crate::Readable for SgpioSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio::W`](W) writer structure"]
impl crate::Writable for SgpioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO to value 0"]
impl crate::Resettable for SgpioSpec {}
