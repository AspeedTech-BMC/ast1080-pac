#[doc = "Register `SCUE1C` reader"]
pub type R = crate::R<Scue1cSpec>;
#[doc = "Register `SCUE1C` writer"]
pub type W = crate::W<Scue1cSpec>;
#[doc = "Field `SCUREGLOCK380` reader - SCU_REG_LOCK_380"]
pub type Scureglock380R = crate::BitReader;
#[doc = "Field `SCUREGLOCK380` writer - SCU_REG_LOCK_380"]
pub type Scureglock380W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK384` reader - SCU_REG_LOCK_384"]
pub type Scureglock384R = crate::BitReader;
#[doc = "Field `SCUREGLOCK384` writer - SCU_REG_LOCK_384"]
pub type Scureglock384W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK390` reader - SCU_REG_LOCK_390"]
pub type Scureglock390R = crate::BitReader;
#[doc = "Field `SCUREGLOCK390` writer - SCU_REG_LOCK_390"]
pub type Scureglock390W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK394` reader - SCU_REG_LOCK_394"]
pub type Scureglock394R = crate::BitReader;
#[doc = "Field `SCUREGLOCK394` writer - SCU_REG_LOCK_394"]
pub type Scureglock394W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK398` reader - SCU_REG_LOCK_398"]
pub type Scureglock398R = crate::BitReader;
#[doc = "Field `SCUREGLOCK398` writer - SCU_REG_LOCK_398"]
pub type Scureglock398W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUREGLOCK3A0` reader - SCU_REG_LOCK_3A0"]
pub type Scureglock3a0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK3A0` writer - SCU_REG_LOCK_3A0"]
pub type Scureglock3a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK3A4` reader - SCU_REG_LOCK_3A4"]
pub type Scureglock3a4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK3A4` writer - SCU_REG_LOCK_3A4"]
pub type Scureglock3a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK3B4` reader - SCU_REG_LOCK_3B4"]
pub type Scureglock3b4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK3B4` writer - SCU_REG_LOCK_3B4"]
pub type Scureglock3b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK3B8` reader - SCU_REG_LOCK_3B8"]
pub type Scureglock3b8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK3B8` writer - SCU_REG_LOCK_3B8"]
pub type Scureglock3b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK3BC` reader - SCU_REG_LOCK_3BC"]
pub type Scureglock3bcR = crate::BitReader;
#[doc = "Field `SCUREGLOCK3BC` writer - SCU_REG_LOCK_3BC"]
pub type Scureglock3bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK3C0` reader - SCU_REG_LOCK_3C0"]
pub type Scureglock3c0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK3C0` writer - SCU_REG_LOCK_3C0"]
pub type Scureglock3c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_380"]
    #[inline(always)]
    pub fn scureglock380(&self) -> Scureglock380R {
        Scureglock380R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_384"]
    #[inline(always)]
    pub fn scureglock384(&self) -> Scureglock384R {
        Scureglock384R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_390"]
    #[inline(always)]
    pub fn scureglock390(&self) -> Scureglock390R {
        Scureglock390R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_394"]
    #[inline(always)]
    pub fn scureglock394(&self) -> Scureglock394R {
        Scureglock394R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_398"]
    #[inline(always)]
    pub fn scureglock398(&self) -> Scureglock398R {
        Scureglock398R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_3A0"]
    #[inline(always)]
    pub fn scureglock3a0(&self) -> Scureglock3a0R {
        Scureglock3a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_3A4"]
    #[inline(always)]
    pub fn scureglock3a4(&self) -> Scureglock3a4R {
        Scureglock3a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_3B4"]
    #[inline(always)]
    pub fn scureglock3b4(&self) -> Scureglock3b4R {
        Scureglock3b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_3B8"]
    #[inline(always)]
    pub fn scureglock3b8(&self) -> Scureglock3b8R {
        Scureglock3b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_3BC"]
    #[inline(always)]
    pub fn scureglock3bc(&self) -> Scureglock3bcR {
        Scureglock3bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_3C0"]
    #[inline(always)]
    pub fn scureglock3c0(&self) -> Scureglock3c0R {
        Scureglock3c0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_380"]
    #[inline(always)]
    pub fn scureglock380(&mut self) -> Scureglock380W<Scue1cSpec> {
        Scureglock380W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_384"]
    #[inline(always)]
    pub fn scureglock384(&mut self) -> Scureglock384W<Scue1cSpec> {
        Scureglock384W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_390"]
    #[inline(always)]
    pub fn scureglock390(&mut self) -> Scureglock390W<Scue1cSpec> {
        Scureglock390W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_394"]
    #[inline(always)]
    pub fn scureglock394(&mut self) -> Scureglock394W<Scue1cSpec> {
        Scureglock394W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_398"]
    #[inline(always)]
    pub fn scureglock398(&mut self) -> Scureglock398W<Scue1cSpec> {
        Scureglock398W::new(self, 6)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_3A0"]
    #[inline(always)]
    pub fn scureglock3a0(&mut self) -> Scureglock3a0W<Scue1cSpec> {
        Scureglock3a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_3A4"]
    #[inline(always)]
    pub fn scureglock3a4(&mut self) -> Scureglock3a4W<Scue1cSpec> {
        Scureglock3a4W::new(self, 9)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_3B4"]
    #[inline(always)]
    pub fn scureglock3b4(&mut self) -> Scureglock3b4W<Scue1cSpec> {
        Scureglock3b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_3B8"]
    #[inline(always)]
    pub fn scureglock3b8(&mut self) -> Scureglock3b8W<Scue1cSpec> {
        Scureglock3b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_3BC"]
    #[inline(always)]
    pub fn scureglock3bc(&mut self) -> Scureglock3bcW<Scue1cSpec> {
        Scureglock3bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_3C0"]
    #[inline(always)]
    pub fn scureglock3c0(&mut self) -> Scureglock3c0W<Scue1cSpec> {
        Scureglock3c0W::new(self, 16)
    }
}
#[doc = "Write Protection 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue1cSpec;
impl crate::RegisterSpec for Scue1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue1c::R`](R) reader structure"]
impl crate::Readable for Scue1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scue1c::W`](W) writer structure"]
impl crate::Writable for Scue1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE1C to value 0"]
impl crate::Resettable for Scue1cSpec {}
