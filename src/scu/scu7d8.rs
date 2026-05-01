#[doc = "Register `SCU7D8` reader"]
pub type R = crate::R<Scu7d8Spec>;
#[doc = "Register `SCU7D8` writer"]
pub type W = crate::W<Scu7d8Spec>;
#[doc = "Field `SCUMUXSEC3IO192` reader - SCU_MUX_SEC3_IO192"]
pub type Scumuxsec3io192R = crate::BitReader;
#[doc = "Field `SCUMUXSEC3IO192` writer - SCU_MUX_SEC3_IO192"]
pub type Scumuxsec3io192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUMUXSEC3IO193` reader - SCU_MUX_SEC3_IO193"]
pub type Scumuxsec3io193R = crate::BitReader;
#[doc = "Field `SCUMUXSEC3IO193` writer - SCU_MUX_SEC3_IO193"]
pub type Scumuxsec3io193W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_MUX_SEC3_IO192"]
    #[inline(always)]
    pub fn scumuxsec3io192(&self) -> Scumuxsec3io192R {
        Scumuxsec3io192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_MUX_SEC3_IO193"]
    #[inline(always)]
    pub fn scumuxsec3io193(&self) -> Scumuxsec3io193R {
        Scumuxsec3io193R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_MUX_SEC3_IO192"]
    #[inline(always)]
    pub fn scumuxsec3io192(&mut self) -> Scumuxsec3io192W<Scu7d8Spec> {
        Scumuxsec3io192W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_MUX_SEC3_IO193"]
    #[inline(always)]
    pub fn scumuxsec3io193(&mut self) -> Scumuxsec3io193W<Scu7d8Spec> {
        Scumuxsec3io193W::new(self, 1)
    }
}
#[doc = "IO Secure 3 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu7d8Spec;
impl crate::RegisterSpec for Scu7d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu7d8::R`](R) reader structure"]
impl crate::Readable for Scu7d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu7d8::W`](W) writer structure"]
impl crate::Writable for Scu7d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU7D8 to value 0"]
impl crate::Resettable for Scu7d8Spec {}
