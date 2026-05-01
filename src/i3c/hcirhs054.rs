#[doc = "Register `HCIRHS054` reader"]
pub type R = crate::R<Hcirhs054Spec>;
#[doc = "Register `HCIRHS054` writer"]
pub type W = crate::W<Hcirhs054Spec>;
#[doc = "Field `REGRINGENABLE` reader - REG_RING_ENABLE"]
pub type RegringenableR = crate::BitReader;
#[doc = "Field `REGRINGENABLE` writer - REG_RING_ENABLE"]
pub type RegringenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGRS` reader - REG_RING_RS"]
pub type RegringrsR = crate::BitReader;
#[doc = "Field `REGRINGRS` writer - REG_RING_RS"]
pub type RegringrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGABORT` reader - REG_RING_ABORT"]
pub type RegringabortR = crate::BitReader;
#[doc = "Field `REGRINGABORT` writer - REG_RING_ABORT"]
pub type RegringabortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_RING_ENABLE"]
    #[inline(always)]
    pub fn regringenable(&self) -> RegringenableR {
        RegringenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_RING_RS"]
    #[inline(always)]
    pub fn regringrs(&self) -> RegringrsR {
        RegringrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_RING_ABORT"]
    #[inline(always)]
    pub fn regringabort(&self) -> RegringabortR {
        RegringabortR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_RING_ENABLE"]
    #[inline(always)]
    pub fn regringenable(&mut self) -> RegringenableW<Hcirhs054Spec> {
        RegringenableW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_RING_RS"]
    #[inline(always)]
    pub fn regringrs(&mut self) -> RegringrsW<Hcirhs054Spec> {
        RegringrsW::new(self, 1)
    }
    #[doc = "Bit 2 - REG_RING_ABORT"]
    #[inline(always)]
    pub fn regringabort(&mut self) -> RegringabortW<Hcirhs054Spec> {
        RegringabortW::new(self, 2)
    }
}
#[doc = "RH\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs054Spec;
impl crate::RegisterSpec for Hcirhs054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs054::R`](R) reader structure"]
impl crate::Readable for Hcirhs054Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs054::W`](W) writer structure"]
impl crate::Writable for Hcirhs054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS054 to value 0"]
impl crate::Resettable for Hcirhs054Spec {}
