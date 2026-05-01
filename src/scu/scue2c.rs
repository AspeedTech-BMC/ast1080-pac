#[doc = "Register `SCUE2C` reader"]
pub type R = crate::R<Scue2cSpec>;
#[doc = "Register `SCUE2C` writer"]
pub type W = crate::W<Scue2cSpec>;
#[doc = "Field `SCUREGLOCK580` reader - SCU_REG_LOCK_580"]
pub type Scureglock580R = crate::BitReader;
#[doc = "Field `SCUREGLOCK580` writer - SCU_REG_LOCK_580"]
pub type Scureglock580W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK584` reader - SCU_REG_LOCK_584"]
pub type Scureglock584R = crate::BitReader;
#[doc = "Field `SCUREGLOCK584` writer - SCU_REG_LOCK_584"]
pub type Scureglock584W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK588` reader - SCU_REG_LOCK_588"]
pub type Scureglock588R = crate::BitReader;
#[doc = "Field `SCUREGLOCK588` writer - SCU_REG_LOCK_588"]
pub type Scureglock588W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK58C` reader - SCU_REG_LOCK_58C"]
pub type Scureglock58cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK58C` writer - SCU_REG_LOCK_58C"]
pub type Scureglock58cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK590` reader - SCU_REG_LOCK_590"]
pub type Scureglock590R = crate::BitReader;
#[doc = "Field `SCUREGLOCK590` writer - SCU_REG_LOCK_590"]
pub type Scureglock590W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK594` reader - SCU_REG_LOCK_594"]
pub type Scureglock594R = crate::BitReader;
#[doc = "Field `SCUREGLOCK594` writer - SCU_REG_LOCK_594"]
pub type Scureglock594W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK598` reader - SCU_REG_LOCK_598"]
pub type Scureglock598R = crate::BitReader;
#[doc = "Field `SCUREGLOCK598` writer - SCU_REG_LOCK_598"]
pub type Scureglock598W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK59C` reader - SCU_REG_LOCK_59C"]
pub type Scureglock59cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK59C` writer - SCU_REG_LOCK_59C"]
pub type Scureglock59cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK5A0` reader - SCU_REG_LOCK_5A0"]
pub type Scureglock5a0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK5A0` writer - SCU_REG_LOCK_5A0"]
pub type Scureglock5a0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_580"]
    #[inline(always)]
    pub fn scureglock580(&self) -> Scureglock580R {
        Scureglock580R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_584"]
    #[inline(always)]
    pub fn scureglock584(&self) -> Scureglock584R {
        Scureglock584R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_588"]
    #[inline(always)]
    pub fn scureglock588(&self) -> Scureglock588R {
        Scureglock588R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_58C"]
    #[inline(always)]
    pub fn scureglock58c(&self) -> Scureglock58cR {
        Scureglock58cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_590"]
    #[inline(always)]
    pub fn scureglock590(&self) -> Scureglock590R {
        Scureglock590R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_594"]
    #[inline(always)]
    pub fn scureglock594(&self) -> Scureglock594R {
        Scureglock594R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_598"]
    #[inline(always)]
    pub fn scureglock598(&self) -> Scureglock598R {
        Scureglock598R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_59C"]
    #[inline(always)]
    pub fn scureglock59c(&self) -> Scureglock59cR {
        Scureglock59cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_5A0"]
    #[inline(always)]
    pub fn scureglock5a0(&self) -> Scureglock5a0R {
        Scureglock5a0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_580"]
    #[inline(always)]
    pub fn scureglock580(&mut self) -> Scureglock580W<Scue2cSpec> {
        Scureglock580W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_584"]
    #[inline(always)]
    pub fn scureglock584(&mut self) -> Scureglock584W<Scue2cSpec> {
        Scureglock584W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_588"]
    #[inline(always)]
    pub fn scureglock588(&mut self) -> Scureglock588W<Scue2cSpec> {
        Scureglock588W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_58C"]
    #[inline(always)]
    pub fn scureglock58c(&mut self) -> Scureglock58cW<Scue2cSpec> {
        Scureglock58cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_590"]
    #[inline(always)]
    pub fn scureglock590(&mut self) -> Scureglock590W<Scue2cSpec> {
        Scureglock590W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_594"]
    #[inline(always)]
    pub fn scureglock594(&mut self) -> Scureglock594W<Scue2cSpec> {
        Scureglock594W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_598"]
    #[inline(always)]
    pub fn scureglock598(&mut self) -> Scureglock598W<Scue2cSpec> {
        Scureglock598W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_59C"]
    #[inline(always)]
    pub fn scureglock59c(&mut self) -> Scureglock59cW<Scue2cSpec> {
        Scureglock59cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_5A0"]
    #[inline(always)]
    pub fn scureglock5a0(&mut self) -> Scureglock5a0W<Scue2cSpec> {
        Scureglock5a0W::new(self, 8)
    }
}
#[doc = "Write Protection 12 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue2cSpec;
impl crate::RegisterSpec for Scue2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue2c::R`](R) reader structure"]
impl crate::Readable for Scue2cSpec {}
#[doc = "`write(|w| ..)` method takes [`scue2c::W`](W) writer structure"]
impl crate::Writable for Scue2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE2C to value 0"]
impl crate::Resettable for Scue2cSpec {}
