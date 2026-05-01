#[doc = "Register `SGPIO04C` reader"]
pub type R = crate::R<Sgpio04cSpec>;
#[doc = "Register `SGPIO04C` writer"]
pub type W = crate::W<Sgpio04cSpec>;
#[doc = "Field `INTStatusOfSGPIO96` reader - Interrupt Status of SGPIO_96"]
pub type IntstatusOfSgpio96R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO97` reader - Interrupt Status of SGPIO_97"]
pub type IntstatusOfSgpio97R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO98` reader - Interrupt Status of SGPIO_98"]
pub type IntstatusOfSgpio98R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO99` reader - Interrupt Status of SGPIO_99"]
pub type IntstatusOfSgpio99R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO100` reader - Interrupt Status of SGPIO_100"]
pub type IntstatusOfSgpio100R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO101` reader - Interrupt Status of SGPIO_101"]
pub type IntstatusOfSgpio101R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO102` reader - Interrupt Status of SGPIO_102"]
pub type IntstatusOfSgpio102R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO103` reader - Interrupt Status of SGPIO_103"]
pub type IntstatusOfSgpio103R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO104` reader - Interrupt Status of SGPIO_104"]
pub type IntstatusOfSgpio104R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO105` reader - Interrupt Status of SGPIO_105"]
pub type IntstatusOfSgpio105R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO106` reader - Interrupt Status of SGPIO_106"]
pub type IntstatusOfSgpio106R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO107` reader - Interrupt Status of SGPIO_107"]
pub type IntstatusOfSgpio107R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO108` reader - Interrupt Status of SGPIO_108"]
pub type IntstatusOfSgpio108R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO109` reader - Interrupt Status of SGPIO_109"]
pub type IntstatusOfSgpio109R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO110` reader - Interrupt Status of SGPIO_110"]
pub type IntstatusOfSgpio110R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO111` reader - Interrupt Status of SGPIO_111"]
pub type IntstatusOfSgpio111R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO112` reader - Interrupt Status of SGPIO_112"]
pub type IntstatusOfSgpio112R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO113` reader - Interrupt Status of SGPIO_113"]
pub type IntstatusOfSgpio113R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO114` reader - Interrupt Status of SGPIO_114"]
pub type IntstatusOfSgpio114R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO115` reader - Interrupt Status of SGPIO_115"]
pub type IntstatusOfSgpio115R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO116` reader - Interrupt Status of SGPIO_116"]
pub type IntstatusOfSgpio116R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO117` reader - Interrupt Status of SGPIO_117"]
pub type IntstatusOfSgpio117R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO118` reader - Interrupt Status of SGPIO_118"]
pub type IntstatusOfSgpio118R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO119` reader - Interrupt Status of SGPIO_119"]
pub type IntstatusOfSgpio119R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO120` reader - Interrupt Status of SGPIO_120"]
pub type IntstatusOfSgpio120R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO121` reader - Interrupt Status of SGPIO_121"]
pub type IntstatusOfSgpio121R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO122` reader - Interrupt Status of SGPIO_122"]
pub type IntstatusOfSgpio122R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO123` reader - Interrupt Status of SGPIO_123"]
pub type IntstatusOfSgpio123R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO124` reader - Interrupt Status of SGPIO_124"]
pub type IntstatusOfSgpio124R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO125` reader - Interrupt Status of SGPIO_125"]
pub type IntstatusOfSgpio125R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO126` reader - Interrupt Status of SGPIO_126"]
pub type IntstatusOfSgpio126R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO127` reader - Interrupt Status of SGPIO_127"]
pub type IntstatusOfSgpio127R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_96"]
    #[inline(always)]
    pub fn intstatus_of_sgpio96(&self) -> IntstatusOfSgpio96R {
        IntstatusOfSgpio96R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_97"]
    #[inline(always)]
    pub fn intstatus_of_sgpio97(&self) -> IntstatusOfSgpio97R {
        IntstatusOfSgpio97R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_98"]
    #[inline(always)]
    pub fn intstatus_of_sgpio98(&self) -> IntstatusOfSgpio98R {
        IntstatusOfSgpio98R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_99"]
    #[inline(always)]
    pub fn intstatus_of_sgpio99(&self) -> IntstatusOfSgpio99R {
        IntstatusOfSgpio99R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_100"]
    #[inline(always)]
    pub fn intstatus_of_sgpio100(&self) -> IntstatusOfSgpio100R {
        IntstatusOfSgpio100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_101"]
    #[inline(always)]
    pub fn intstatus_of_sgpio101(&self) -> IntstatusOfSgpio101R {
        IntstatusOfSgpio101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_102"]
    #[inline(always)]
    pub fn intstatus_of_sgpio102(&self) -> IntstatusOfSgpio102R {
        IntstatusOfSgpio102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_103"]
    #[inline(always)]
    pub fn intstatus_of_sgpio103(&self) -> IntstatusOfSgpio103R {
        IntstatusOfSgpio103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_104"]
    #[inline(always)]
    pub fn intstatus_of_sgpio104(&self) -> IntstatusOfSgpio104R {
        IntstatusOfSgpio104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_105"]
    #[inline(always)]
    pub fn intstatus_of_sgpio105(&self) -> IntstatusOfSgpio105R {
        IntstatusOfSgpio105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_106"]
    #[inline(always)]
    pub fn intstatus_of_sgpio106(&self) -> IntstatusOfSgpio106R {
        IntstatusOfSgpio106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_107"]
    #[inline(always)]
    pub fn intstatus_of_sgpio107(&self) -> IntstatusOfSgpio107R {
        IntstatusOfSgpio107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_108"]
    #[inline(always)]
    pub fn intstatus_of_sgpio108(&self) -> IntstatusOfSgpio108R {
        IntstatusOfSgpio108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_109"]
    #[inline(always)]
    pub fn intstatus_of_sgpio109(&self) -> IntstatusOfSgpio109R {
        IntstatusOfSgpio109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_110"]
    #[inline(always)]
    pub fn intstatus_of_sgpio110(&self) -> IntstatusOfSgpio110R {
        IntstatusOfSgpio110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_111"]
    #[inline(always)]
    pub fn intstatus_of_sgpio111(&self) -> IntstatusOfSgpio111R {
        IntstatusOfSgpio111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_112"]
    #[inline(always)]
    pub fn intstatus_of_sgpio112(&self) -> IntstatusOfSgpio112R {
        IntstatusOfSgpio112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_113"]
    #[inline(always)]
    pub fn intstatus_of_sgpio113(&self) -> IntstatusOfSgpio113R {
        IntstatusOfSgpio113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_114"]
    #[inline(always)]
    pub fn intstatus_of_sgpio114(&self) -> IntstatusOfSgpio114R {
        IntstatusOfSgpio114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_115"]
    #[inline(always)]
    pub fn intstatus_of_sgpio115(&self) -> IntstatusOfSgpio115R {
        IntstatusOfSgpio115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_116"]
    #[inline(always)]
    pub fn intstatus_of_sgpio116(&self) -> IntstatusOfSgpio116R {
        IntstatusOfSgpio116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_117"]
    #[inline(always)]
    pub fn intstatus_of_sgpio117(&self) -> IntstatusOfSgpio117R {
        IntstatusOfSgpio117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_118"]
    #[inline(always)]
    pub fn intstatus_of_sgpio118(&self) -> IntstatusOfSgpio118R {
        IntstatusOfSgpio118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_119"]
    #[inline(always)]
    pub fn intstatus_of_sgpio119(&self) -> IntstatusOfSgpio119R {
        IntstatusOfSgpio119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_120"]
    #[inline(always)]
    pub fn intstatus_of_sgpio120(&self) -> IntstatusOfSgpio120R {
        IntstatusOfSgpio120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_121"]
    #[inline(always)]
    pub fn intstatus_of_sgpio121(&self) -> IntstatusOfSgpio121R {
        IntstatusOfSgpio121R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_122"]
    #[inline(always)]
    pub fn intstatus_of_sgpio122(&self) -> IntstatusOfSgpio122R {
        IntstatusOfSgpio122R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_123"]
    #[inline(always)]
    pub fn intstatus_of_sgpio123(&self) -> IntstatusOfSgpio123R {
        IntstatusOfSgpio123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_124"]
    #[inline(always)]
    pub fn intstatus_of_sgpio124(&self) -> IntstatusOfSgpio124R {
        IntstatusOfSgpio124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_125"]
    #[inline(always)]
    pub fn intstatus_of_sgpio125(&self) -> IntstatusOfSgpio125R {
        IntstatusOfSgpio125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_126"]
    #[inline(always)]
    pub fn intstatus_of_sgpio126(&self) -> IntstatusOfSgpio126R {
        IntstatusOfSgpio126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_127"]
    #[inline(always)]
    pub fn intstatus_of_sgpio127(&self) -> IntstatusOfSgpio127R {
        IntstatusOfSgpio127R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio04cSpec;
impl crate::RegisterSpec for Sgpio04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio04c::R`](R) reader structure"]
impl crate::Readable for Sgpio04cSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio04c::W`](W) writer structure"]
impl crate::Writable for Sgpio04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO04C to value 0"]
impl crate::Resettable for Sgpio04cSpec {}
