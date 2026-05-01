#[doc = "Register `SCUF1C` reader"]
pub type R = crate::R<Scuf1cSpec>;
#[doc = "Register `SCUF1C` writer"]
pub type W = crate::W<Scuf1cSpec>;
#[doc = "Field `SCUREGRST380` reader - SCU_REG_RST_380"]
pub type Scuregrst380R = crate::BitReader;
#[doc = "Field `SCUREGRST380` writer - SCU_REG_RST_380"]
pub type Scuregrst380W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST384` reader - SCU_REG_RST_384"]
pub type Scuregrst384R = crate::BitReader;
#[doc = "Field `SCUREGRST384` writer - SCU_REG_RST_384"]
pub type Scuregrst384W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGRST390` reader - SCU_REG_RST_390"]
pub type Scuregrst390R = crate::BitReader;
#[doc = "Field `SCUREGRST390` writer - SCU_REG_RST_390"]
pub type Scuregrst390W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST394` reader - SCU_REG_RST_394"]
pub type Scuregrst394R = crate::BitReader;
#[doc = "Field `SCUREGRST394` writer - SCU_REG_RST_394"]
pub type Scuregrst394W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST398` reader - SCU_REG_RST_398"]
pub type Scuregrst398R = crate::BitReader;
#[doc = "Field `SCUREGRST398` writer - SCU_REG_RST_398"]
pub type Scuregrst398W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUREGRST3A0` reader - SCU_REG_RST_3A0"]
pub type Scuregrst3a0R = crate::BitReader;
#[doc = "Field `SCUREGRST3A0` writer - SCU_REG_RST_3A0"]
pub type Scuregrst3a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST3A4` reader - SCU_REG_RST_3A4"]
pub type Scuregrst3a4R = crate::BitReader;
#[doc = "Field `SCUREGRST3A4` writer - SCU_REG_RST_3A4"]
pub type Scuregrst3a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGRST3B0` reader - SCU_REG_RST_3B0"]
pub type Scuregrst3b0R = crate::BitReader;
#[doc = "Field `SCUREGRST3B0` writer - SCU_REG_RST_3B0"]
pub type Scuregrst3b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST3B4` reader - SCU_REG_RST_3B4"]
pub type Scuregrst3b4R = crate::BitReader;
#[doc = "Field `SCUREGRST3B4` writer - SCU_REG_RST_3B4"]
pub type Scuregrst3b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST3B8` reader - SCU_REG_RST_3B8"]
pub type Scuregrst3b8R = crate::BitReader;
#[doc = "Field `SCUREGRST3B8` writer - SCU_REG_RST_3B8"]
pub type Scuregrst3b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST3BC` reader - SCU_REG_RST_3BC"]
pub type Scuregrst3bcR = crate::BitReader;
#[doc = "Field `SCUREGRST3BC` writer - SCU_REG_RST_3BC"]
pub type Scuregrst3bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST3C0` reader - SCU_REG_RST_3C0"]
pub type Scuregrst3c0R = crate::BitReader;
#[doc = "Field `SCUREGRST3C0` writer - SCU_REG_RST_3C0"]
pub type Scuregrst3c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_380"]
    #[inline(always)]
    pub fn scuregrst380(&self) -> Scuregrst380R {
        Scuregrst380R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_384"]
    #[inline(always)]
    pub fn scuregrst384(&self) -> Scuregrst384R {
        Scuregrst384R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_RST_390"]
    #[inline(always)]
    pub fn scuregrst390(&self) -> Scuregrst390R {
        Scuregrst390R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_394"]
    #[inline(always)]
    pub fn scuregrst394(&self) -> Scuregrst394R {
        Scuregrst394R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_398"]
    #[inline(always)]
    pub fn scuregrst398(&self) -> Scuregrst398R {
        Scuregrst398R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_RST_3A0"]
    #[inline(always)]
    pub fn scuregrst3a0(&self) -> Scuregrst3a0R {
        Scuregrst3a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_RST_3A4"]
    #[inline(always)]
    pub fn scuregrst3a4(&self) -> Scuregrst3a4R {
        Scuregrst3a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_RST_3B0"]
    #[inline(always)]
    pub fn scuregrst3b0(&self) -> Scuregrst3b0R {
        Scuregrst3b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_RST_3B4"]
    #[inline(always)]
    pub fn scuregrst3b4(&self) -> Scuregrst3b4R {
        Scuregrst3b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_RST_3B8"]
    #[inline(always)]
    pub fn scuregrst3b8(&self) -> Scuregrst3b8R {
        Scuregrst3b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_RST_3BC"]
    #[inline(always)]
    pub fn scuregrst3bc(&self) -> Scuregrst3bcR {
        Scuregrst3bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_RST_3C0"]
    #[inline(always)]
    pub fn scuregrst3c0(&self) -> Scuregrst3c0R {
        Scuregrst3c0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_380"]
    #[inline(always)]
    pub fn scuregrst380(&mut self) -> Scuregrst380W<Scuf1cSpec> {
        Scuregrst380W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_384"]
    #[inline(always)]
    pub fn scuregrst384(&mut self) -> Scuregrst384W<Scuf1cSpec> {
        Scuregrst384W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_RST_390"]
    #[inline(always)]
    pub fn scuregrst390(&mut self) -> Scuregrst390W<Scuf1cSpec> {
        Scuregrst390W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_394"]
    #[inline(always)]
    pub fn scuregrst394(&mut self) -> Scuregrst394W<Scuf1cSpec> {
        Scuregrst394W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_398"]
    #[inline(always)]
    pub fn scuregrst398(&mut self) -> Scuregrst398W<Scuf1cSpec> {
        Scuregrst398W::new(self, 6)
    }
    #[doc = "Bit 8 - SCU_REG_RST_3A0"]
    #[inline(always)]
    pub fn scuregrst3a0(&mut self) -> Scuregrst3a0W<Scuf1cSpec> {
        Scuregrst3a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_RST_3A4"]
    #[inline(always)]
    pub fn scuregrst3a4(&mut self) -> Scuregrst3a4W<Scuf1cSpec> {
        Scuregrst3a4W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_RST_3B0"]
    #[inline(always)]
    pub fn scuregrst3b0(&mut self) -> Scuregrst3b0W<Scuf1cSpec> {
        Scuregrst3b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_RST_3B4"]
    #[inline(always)]
    pub fn scuregrst3b4(&mut self) -> Scuregrst3b4W<Scuf1cSpec> {
        Scuregrst3b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_RST_3B8"]
    #[inline(always)]
    pub fn scuregrst3b8(&mut self) -> Scuregrst3b8W<Scuf1cSpec> {
        Scuregrst3b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_RST_3BC"]
    #[inline(always)]
    pub fn scuregrst3bc(&mut self) -> Scuregrst3bcW<Scuf1cSpec> {
        Scuregrst3bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_RST_3C0"]
    #[inline(always)]
    pub fn scuregrst3c0(&mut self) -> Scuregrst3c0W<Scuf1cSpec> {
        Scuregrst3c0W::new(self, 16)
    }
}
#[doc = "Reset Control 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf1cSpec;
impl crate::RegisterSpec for Scuf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf1c::R`](R) reader structure"]
impl crate::Readable for Scuf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf1c::W`](W) writer structure"]
impl crate::Writable for Scuf1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF1C to value 0"]
impl crate::Resettable for Scuf1cSpec {}
