#[doc = "Register `GSRAM008` reader"]
pub type R = crate::R<Gsram008Spec>;
#[doc = "Register `GSRAM008` writer"]
pub type W = crate::W<Gsram008Spec>;
#[doc = "Field `WLOCKEN` reader - WLOCK_EN"]
pub type WlockenR = crate::BitReader;
#[doc = "Field `WLOCKEN` writer - WLOCK_EN"]
pub type WlockenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPROTALL` reader - WPROT_ALL"]
pub type WprotallR = crate::BitReader;
#[doc = "Field `WPROTALL` writer - WPROT_ALL"]
pub type WprotallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WLOCK_EN"]
    #[inline(always)]
    pub fn wlocken(&self) -> WlockenR {
        WlockenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WPROT_ALL"]
    #[inline(always)]
    pub fn wprotall(&self) -> WprotallR {
        WprotallR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WLOCK_EN"]
    #[inline(always)]
    pub fn wlocken(&mut self) -> WlockenW<Gsram008Spec> {
        WlockenW::new(self, 0)
    }
    #[doc = "Bit 1 - WPROT_ALL"]
    #[inline(always)]
    pub fn wprotall(&mut self) -> WprotallW<Gsram008Spec> {
        WprotallW::new(self, 1)
    }
}
#[doc = "GSRAM\\_WP\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram008Spec;
impl crate::RegisterSpec for Gsram008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram008::R`](R) reader structure"]
impl crate::Readable for Gsram008Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram008::W`](W) writer structure"]
impl crate::Writable for Gsram008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM008 to value 0"]
impl crate::Resettable for Gsram008Spec {}
