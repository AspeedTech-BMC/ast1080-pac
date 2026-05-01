#[doc = "Register `SGPIO30` reader"]
pub type R = crate::R<Sgpio30Spec>;
#[doc = "Register `SGPIO30` writer"]
pub type W = crate::W<Sgpio30Spec>;
#[doc = "Field `OutputValueOfSGPIO96` reader - Output value of SGPIO_96"]
pub type OutputValueOfSgpio96R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO97` reader - Output value of SGPIO_97"]
pub type OutputValueOfSgpio97R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO98` reader - Output value of SGPIO_98"]
pub type OutputValueOfSgpio98R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO99` reader - Output value of SGPIO_99"]
pub type OutputValueOfSgpio99R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO100` reader - Output value of SGPIO_100"]
pub type OutputValueOfSgpio100R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO101` reader - Output value of SGPIO_101"]
pub type OutputValueOfSgpio101R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO102` reader - Output value of SGPIO_102"]
pub type OutputValueOfSgpio102R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO103` reader - Output value of SGPIO_103"]
pub type OutputValueOfSgpio103R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO104` reader - Output value of SGPIO_104"]
pub type OutputValueOfSgpio104R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO105` reader - Output value of SGPIO_105"]
pub type OutputValueOfSgpio105R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO106` reader - Output value of SGPIO_106"]
pub type OutputValueOfSgpio106R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO107` reader - Output value of SGPIO_107"]
pub type OutputValueOfSgpio107R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO108` reader - Output value of SGPIO_108"]
pub type OutputValueOfSgpio108R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO109` reader - Output value of SGPIO_109"]
pub type OutputValueOfSgpio109R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO110` reader - Output value of SGPIO_110"]
pub type OutputValueOfSgpio110R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO111` reader - Output value of SGPIO_111"]
pub type OutputValueOfSgpio111R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO112` reader - Output value of SGPIO_112"]
pub type OutputValueOfSgpio112R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO113` reader - Output value of SGPIO_113"]
pub type OutputValueOfSgpio113R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO114` reader - Output value of SGPIO_114"]
pub type OutputValueOfSgpio114R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO115` reader - Output value of SGPIO_115"]
pub type OutputValueOfSgpio115R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO116` reader - Output value of SGPIO_116"]
pub type OutputValueOfSgpio116R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO117` reader - Output value of SGPIO_117"]
pub type OutputValueOfSgpio117R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO118` reader - Output value of SGPIO_118"]
pub type OutputValueOfSgpio118R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO119` reader - Output value of SGPIO_119"]
pub type OutputValueOfSgpio119R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO120` reader - Output value of SGPIO_120"]
pub type OutputValueOfSgpio120R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO121` reader - Output value of SGPIO_121"]
pub type OutputValueOfSgpio121R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO122` reader - Output value of SGPIO_122"]
pub type OutputValueOfSgpio122R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO123` reader - Output value of SGPIO_123"]
pub type OutputValueOfSgpio123R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO124` reader - Output value of SGPIO_124"]
pub type OutputValueOfSgpio124R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO125` reader - Output value of SGPIO_125"]
pub type OutputValueOfSgpio125R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO126` reader - Output value of SGPIO_126"]
pub type OutputValueOfSgpio126R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO127` reader - Output value of SGPIO_127"]
pub type OutputValueOfSgpio127R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output value of SGPIO_96"]
    #[inline(always)]
    pub fn output_value_of_sgpio96(&self) -> OutputValueOfSgpio96R {
        OutputValueOfSgpio96R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output value of SGPIO_97"]
    #[inline(always)]
    pub fn output_value_of_sgpio97(&self) -> OutputValueOfSgpio97R {
        OutputValueOfSgpio97R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output value of SGPIO_98"]
    #[inline(always)]
    pub fn output_value_of_sgpio98(&self) -> OutputValueOfSgpio98R {
        OutputValueOfSgpio98R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output value of SGPIO_99"]
    #[inline(always)]
    pub fn output_value_of_sgpio99(&self) -> OutputValueOfSgpio99R {
        OutputValueOfSgpio99R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output value of SGPIO_100"]
    #[inline(always)]
    pub fn output_value_of_sgpio100(&self) -> OutputValueOfSgpio100R {
        OutputValueOfSgpio100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output value of SGPIO_101"]
    #[inline(always)]
    pub fn output_value_of_sgpio101(&self) -> OutputValueOfSgpio101R {
        OutputValueOfSgpio101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output value of SGPIO_102"]
    #[inline(always)]
    pub fn output_value_of_sgpio102(&self) -> OutputValueOfSgpio102R {
        OutputValueOfSgpio102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output value of SGPIO_103"]
    #[inline(always)]
    pub fn output_value_of_sgpio103(&self) -> OutputValueOfSgpio103R {
        OutputValueOfSgpio103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output value of SGPIO_104"]
    #[inline(always)]
    pub fn output_value_of_sgpio104(&self) -> OutputValueOfSgpio104R {
        OutputValueOfSgpio104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output value of SGPIO_105"]
    #[inline(always)]
    pub fn output_value_of_sgpio105(&self) -> OutputValueOfSgpio105R {
        OutputValueOfSgpio105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output value of SGPIO_106"]
    #[inline(always)]
    pub fn output_value_of_sgpio106(&self) -> OutputValueOfSgpio106R {
        OutputValueOfSgpio106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output value of SGPIO_107"]
    #[inline(always)]
    pub fn output_value_of_sgpio107(&self) -> OutputValueOfSgpio107R {
        OutputValueOfSgpio107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output value of SGPIO_108"]
    #[inline(always)]
    pub fn output_value_of_sgpio108(&self) -> OutputValueOfSgpio108R {
        OutputValueOfSgpio108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output value of SGPIO_109"]
    #[inline(always)]
    pub fn output_value_of_sgpio109(&self) -> OutputValueOfSgpio109R {
        OutputValueOfSgpio109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output value of SGPIO_110"]
    #[inline(always)]
    pub fn output_value_of_sgpio110(&self) -> OutputValueOfSgpio110R {
        OutputValueOfSgpio110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output value of SGPIO_111"]
    #[inline(always)]
    pub fn output_value_of_sgpio111(&self) -> OutputValueOfSgpio111R {
        OutputValueOfSgpio111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output value of SGPIO_112"]
    #[inline(always)]
    pub fn output_value_of_sgpio112(&self) -> OutputValueOfSgpio112R {
        OutputValueOfSgpio112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output value of SGPIO_113"]
    #[inline(always)]
    pub fn output_value_of_sgpio113(&self) -> OutputValueOfSgpio113R {
        OutputValueOfSgpio113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output value of SGPIO_114"]
    #[inline(always)]
    pub fn output_value_of_sgpio114(&self) -> OutputValueOfSgpio114R {
        OutputValueOfSgpio114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output value of SGPIO_115"]
    #[inline(always)]
    pub fn output_value_of_sgpio115(&self) -> OutputValueOfSgpio115R {
        OutputValueOfSgpio115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output value of SGPIO_116"]
    #[inline(always)]
    pub fn output_value_of_sgpio116(&self) -> OutputValueOfSgpio116R {
        OutputValueOfSgpio116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output value of SGPIO_117"]
    #[inline(always)]
    pub fn output_value_of_sgpio117(&self) -> OutputValueOfSgpio117R {
        OutputValueOfSgpio117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output value of SGPIO_118"]
    #[inline(always)]
    pub fn output_value_of_sgpio118(&self) -> OutputValueOfSgpio118R {
        OutputValueOfSgpio118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output value of SGPIO_119"]
    #[inline(always)]
    pub fn output_value_of_sgpio119(&self) -> OutputValueOfSgpio119R {
        OutputValueOfSgpio119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Output value of SGPIO_120"]
    #[inline(always)]
    pub fn output_value_of_sgpio120(&self) -> OutputValueOfSgpio120R {
        OutputValueOfSgpio120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Output value of SGPIO_121"]
    #[inline(always)]
    pub fn output_value_of_sgpio121(&self) -> OutputValueOfSgpio121R {
        OutputValueOfSgpio121R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Output value of SGPIO_122"]
    #[inline(always)]
    pub fn output_value_of_sgpio122(&self) -> OutputValueOfSgpio122R {
        OutputValueOfSgpio122R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Output value of SGPIO_123"]
    #[inline(always)]
    pub fn output_value_of_sgpio123(&self) -> OutputValueOfSgpio123R {
        OutputValueOfSgpio123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Output value of SGPIO_124"]
    #[inline(always)]
    pub fn output_value_of_sgpio124(&self) -> OutputValueOfSgpio124R {
        OutputValueOfSgpio124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Output value of SGPIO_125"]
    #[inline(always)]
    pub fn output_value_of_sgpio125(&self) -> OutputValueOfSgpio125R {
        OutputValueOfSgpio125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Output value of SGPIO_126"]
    #[inline(always)]
    pub fn output_value_of_sgpio126(&self) -> OutputValueOfSgpio126R {
        OutputValueOfSgpio126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Output value of SGPIO_127"]
    #[inline(always)]
    pub fn output_value_of_sgpio127(&self) -> OutputValueOfSgpio127R {
        OutputValueOfSgpio127R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio30Spec;
impl crate::RegisterSpec for Sgpio30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio30::R`](R) reader structure"]
impl crate::Readable for Sgpio30Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio30::W`](W) writer structure"]
impl crate::Writable for Sgpio30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO30 to value 0"]
impl crate::Resettable for Sgpio30Spec {}
