#[doc = "Register `SCU798` reader"]
pub type R = crate::R<Scu798Spec>;
#[doc = "Register `SCU798` writer"]
pub type W = crate::W<Scu798Spec>;
#[doc = "Field `SCUMUXSEC2IO192` reader - SCU_MUX_SEC2_IO192"]
pub type Scumuxsec2io192R = crate::BitReader;
#[doc = "Field `SCUMUXSEC2IO192` writer - SCU_MUX_SEC2_IO192"]
pub type Scumuxsec2io192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUMUXSEC2IO193` reader - SCU_MUX_SEC2_IO193"]
pub type Scumuxsec2io193R = crate::BitReader;
#[doc = "Field `SCUMUXSEC2IO193` writer - SCU_MUX_SEC2_IO193"]
pub type Scumuxsec2io193W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_MUX_SEC2_IO192"]
    #[inline(always)]
    pub fn scumuxsec2io192(&self) -> Scumuxsec2io192R {
        Scumuxsec2io192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_MUX_SEC2_IO193"]
    #[inline(always)]
    pub fn scumuxsec2io193(&self) -> Scumuxsec2io193R {
        Scumuxsec2io193R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_MUX_SEC2_IO192"]
    #[inline(always)]
    pub fn scumuxsec2io192(&mut self) -> Scumuxsec2io192W<Scu798Spec> {
        Scumuxsec2io192W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_MUX_SEC2_IO193"]
    #[inline(always)]
    pub fn scumuxsec2io193(&mut self) -> Scumuxsec2io193W<Scu798Spec> {
        Scumuxsec2io193W::new(self, 1)
    }
}
#[doc = "IO SECURE 2 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu798::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu798::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu798Spec;
impl crate::RegisterSpec for Scu798Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu798::R`](R) reader structure"]
impl crate::Readable for Scu798Spec {}
#[doc = "`write(|w| ..)` method takes [`scu798::W`](W) writer structure"]
impl crate::Writable for Scu798Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU798 to value 0"]
impl crate::Resettable for Scu798Spec {}
