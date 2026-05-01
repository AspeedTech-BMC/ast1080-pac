#[doc = "Register `AT0FC` reader"]
pub type R = crate::R<At0fcSpec>;
#[doc = "Register `AT0FC` writer"]
pub type W = crate::W<At0fcSpec>;
#[doc = "Field `ATLOCK084` reader - AT_LOCK_084"]
pub type Atlock084R = crate::BitReader;
#[doc = "Field `ATLOCK084` writer - AT_LOCK_084"]
pub type Atlock084W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK088` reader - AT_LOCK_088"]
pub type Atlock088R = crate::BitReader;
#[doc = "Field `ATLOCK088` writer - AT_LOCK_088"]
pub type Atlock088W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK08C` reader - AT_LOCK_08C"]
pub type Atlock08cR = crate::BitReader;
#[doc = "Field `ATLOCK08C` writer - AT_LOCK_08C"]
pub type Atlock08cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK090` reader - AT_LOCK_090"]
pub type Atlock090R = crate::BitReader;
#[doc = "Field `ATLOCK090` writer - AT_LOCK_090"]
pub type Atlock090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK094` reader - AT_LOCK_094"]
pub type Atlock094R = crate::BitReader;
#[doc = "Field `ATLOCK094` writer - AT_LOCK_094"]
pub type Atlock094W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK098` reader - AT_LOCK_098"]
pub type Atlock098R = crate::BitReader;
#[doc = "Field `ATLOCK098` writer - AT_LOCK_098"]
pub type Atlock098W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK09C` reader - AT_LOCK_09C"]
pub type Atlock09cR = crate::BitReader;
#[doc = "Field `ATLOCK09C` writer - AT_LOCK_09C"]
pub type Atlock09cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK0F8` reader - AT_LOCK_0F8"]
pub type Atlock0f8R = crate::BitReader;
#[doc = "Field `ATLOCK0F8` writer - AT_LOCK_0F8"]
pub type Atlock0f8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATLOCK0FC` reader - AT_LOCK_0FC"]
pub type Atlock0fcR = crate::BitReader;
#[doc = "Field `ATLOCK0FC` writer - AT_LOCK_0FC"]
pub type Atlock0fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - AT_LOCK_084"]
    #[inline(always)]
    pub fn atlock084(&self) -> Atlock084R {
        Atlock084R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_LOCK_088"]
    #[inline(always)]
    pub fn atlock088(&self) -> Atlock088R {
        Atlock088R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AT_LOCK_08C"]
    #[inline(always)]
    pub fn atlock08c(&self) -> Atlock08cR {
        Atlock08cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AT_LOCK_090"]
    #[inline(always)]
    pub fn atlock090(&self) -> Atlock090R {
        Atlock090R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AT_LOCK_094"]
    #[inline(always)]
    pub fn atlock094(&self) -> Atlock094R {
        Atlock094R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AT_LOCK_098"]
    #[inline(always)]
    pub fn atlock098(&self) -> Atlock098R {
        Atlock098R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AT_LOCK_09C"]
    #[inline(always)]
    pub fn atlock09c(&self) -> Atlock09cR {
        Atlock09cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 30 - AT_LOCK_0F8"]
    #[inline(always)]
    pub fn atlock0f8(&self) -> Atlock0f8R {
        Atlock0f8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AT_LOCK_0FC"]
    #[inline(always)]
    pub fn atlock0fc(&self) -> Atlock0fcR {
        Atlock0fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - AT_LOCK_084"]
    #[inline(always)]
    pub fn atlock084(&mut self) -> Atlock084W<At0fcSpec> {
        Atlock084W::new(self, 1)
    }
    #[doc = "Bit 2 - AT_LOCK_088"]
    #[inline(always)]
    pub fn atlock088(&mut self) -> Atlock088W<At0fcSpec> {
        Atlock088W::new(self, 2)
    }
    #[doc = "Bit 3 - AT_LOCK_08C"]
    #[inline(always)]
    pub fn atlock08c(&mut self) -> Atlock08cW<At0fcSpec> {
        Atlock08cW::new(self, 3)
    }
    #[doc = "Bit 4 - AT_LOCK_090"]
    #[inline(always)]
    pub fn atlock090(&mut self) -> Atlock090W<At0fcSpec> {
        Atlock090W::new(self, 4)
    }
    #[doc = "Bit 5 - AT_LOCK_094"]
    #[inline(always)]
    pub fn atlock094(&mut self) -> Atlock094W<At0fcSpec> {
        Atlock094W::new(self, 5)
    }
    #[doc = "Bit 6 - AT_LOCK_098"]
    #[inline(always)]
    pub fn atlock098(&mut self) -> Atlock098W<At0fcSpec> {
        Atlock098W::new(self, 6)
    }
    #[doc = "Bit 7 - AT_LOCK_09C"]
    #[inline(always)]
    pub fn atlock09c(&mut self) -> Atlock09cW<At0fcSpec> {
        Atlock09cW::new(self, 7)
    }
    #[doc = "Bit 30 - AT_LOCK_0F8"]
    #[inline(always)]
    pub fn atlock0f8(&mut self) -> Atlock0f8W<At0fcSpec> {
        Atlock0f8W::new(self, 30)
    }
    #[doc = "Bit 31 - AT_LOCK_0FC"]
    #[inline(always)]
    pub fn atlock0fc(&mut self) -> Atlock0fcW<At0fcSpec> {
        Atlock0fcW::new(self, 31)
    }
}
#[doc = "Write Protection 2\n\nYou can [`read`](crate::Reg::read) this register and get [`at0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At0fcSpec;
impl crate::RegisterSpec for At0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at0fc::R`](R) reader structure"]
impl crate::Readable for At0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`at0fc::W`](W) writer structure"]
impl crate::Writable for At0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT0FC to value 0"]
impl crate::Resettable for At0fcSpec {}
